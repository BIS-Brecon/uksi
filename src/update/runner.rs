use std::path::PathBuf;

use mdbsql::Connection;
use sqlx::{Executor, PgTransaction, QueryBuilder, query};

use crate::{
    Aggregate, Individual, Organism, Owner,
    designation::{TaxonDesignation, TaxonDesignationType, TaxonDesignationTypeKind},
    input_group_map::InputGroupMap,
    list::{TaxonList, TaxonListItem, TaxonListType, TaxonListVersion},
    nameserver::Nameserver,
    taxa::{Taxon, TaxonGroupName, TaxonNameType, TaxonRank, TaxonVersion},
    update::{delete, table::Table},
};

const MAX_BUFFER_SIZE: usize = 500;
const BIND_LIMIT: usize = 65535;

#[derive(Debug, thiserror::Error)]
pub enum UpdateError {
    #[error("Error reading data")]
    Read(#[from] mdbsql::Error),
    #[error("Error inserting data")]
    Insert(#[from] sqlx::Error),
}

/// Update the UKSI data from a NHM provided MSAccess file.
/// Wraps the process in a transaction for atomic updates,
/// and to allow application logic to run before it is committed.
/// (e.g. dropping and re-creating foreign key constraints,
/// updating from latest nameserver etc.)
pub async fn update(
    path: PathBuf,
    transaction: &mut PgTransaction<'static>,
) -> Result<(), UpdateError> {
    // Delete existing data.
    delete::delete(transaction).await?;

    // Read Access file
    let conn = Connection::open(path)?;

    // Repopulate
    // Add missing entries to individuals table to fix foreign key constraints.
    // Needed as current UKSI is missing individuals that are referenced in other tables.
    insert_table::<Individual>(&conn, transaction).await?;
    transaction
        .execute(query!(
            r#"
            INSERT INTO uksi.individual(
                name_key,
                comment,
                system_supplied_data
            )
            VALUES (
                'NBNSYS0000000000',
                'Individual missing from UKSI.',
                false
            ),
            (
                'NBNSYS0000000003',
                'Individual missing from UKSI.',
                false
            ),
            (
                'NBNSYS0000000009',
                'Individual missing from UKSI.',
                false
            ),
            (
                'NBNSYS0000000011',
                'Individual missing from UKSI.',
                false
            ),
            (
                'NBNSYS0000000012',
                'Individual missing from UKSI.',
                false
            ),
            (
                'NBNSYS0000000013',
                'Individual missing from UKSI.',
                false
            ),
            (
                'NBNSYS0000000014',
                'Individual missing from UKSI.',
                false
            ),
            (
                'NBNSYS0000000015',
                'Individual missing from UKSI.',
                false
            ),
            (
                'NBNSYS0000000016',
                'Individual missing from UKSI.',
                false
            ),
            (
                'NBNSYS0000000017',
                'Individual missing from UKSI.',
                false
            ),
            (
                'NBNSYS0000000018',
                'Individual missing from UKSI.',
                false
            ),
            (
                'NBNSYS0000000019',
                'Individual missing from UKSI.',
                false
            ),
            (
                'NBNSYS0000000020',
                'Individual missing from UKSI.',
                false
            ),
            (
                'NHMSYS0000000028',
                'Individual missing from UKSI.',
                false
            )
            ON CONFLICT
                DO NOTHING;
    "#
        ))
        .await?;

    // Add missing entry to owner table to fix foreign key constraints.
    // Needed as current UKSI is missing Owner with key 'NHMSYS0021120134'
    // that is referenced in other tables.
    insert_table::<Owner>(&conn, transaction).await?;
    transaction
        .execute(query!(
            r#"
            INSERT INTO uksi.owner (
                owner_key,
                owner_name,
                owner_shortname
            )
            VALUES (
                'NHMSYS0021120134',
                'Owner missing from UKSI.',
                'Missing Owner'
            )
            ON CONFLICT
                DO NOTHING;
    "#
        ))
        .await?;

    insert_table::<TaxonNameType>(&conn, transaction).await?;
    insert_table::<Taxon>(&conn, transaction).await?;
    insert_table::<TaxonRank>(&conn, transaction).await?;
    insert_table::<TaxonGroupName>(&conn, transaction).await?;
    insert_table::<TaxonVersion>(&conn, transaction).await?;
    insert_table::<Nameserver>(&conn, transaction).await?;

    // Drop foreign key constraint as organism is self-referential...
    transaction
        .execute(query!(
            r#"
            ALTER TABLE
                organism
            DROP CONSTRAINT
                organism_parent_key_fkey;
    "#
        ))
        .await?;

    // ... then re-create after import.
    insert_table::<Organism>(&conn, transaction).await?;
    transaction
        .execute(query!(
            r#"
            ALTER TABLE
                organism
            ADD CONSTRAINT
                organism_parent_key_fkey
            FOREIGN KEY (parent_key)
            REFERENCES uksi.organism (
                organism_key
            ) ON DELETE CASCADE;
    "#
        ))
        .await?;

    insert_table::<TaxonListType>(&conn, transaction).await?;
    insert_table::<TaxonList>(&conn, transaction).await?;
    insert_table::<TaxonListVersion>(&conn, transaction).await?;

    // Drop foreign key constraints as taxon_list_item is self-referential...
    transaction
        .execute(query!(
            r#"
            ALTER TABLE
                taxon_list_item
            DROP CONSTRAINT
                taxon_list_item_parent_fkey;
    "#
        ))
        .await?;
    transaction
        .execute(query!(
            r#"
            ALTER TABLE
                taxon_list_item
            DROP CONSTRAINT
                taxon_list_item_preferred_name_fkey;
    "#
        ))
        .await?;

    // ... then re-create after import.
    insert_table::<TaxonListItem>(&conn, transaction).await?;
    transaction
        .execute(query!(
            r#"
            ALTER TABLE
                taxon_list_item
            ADD CONSTRAINT
                taxon_list_item_parent_fkey
            FOREIGN KEY (parent)
            REFERENCES uksi.taxon_list_item (
                taxon_list_item_key
            ) ON DELETE CASCADE;
    "#
        ))
        .await?;
    transaction
        .execute(query!(
            r#"
            ALTER TABLE
                taxon_list_item
            ADD CONSTRAINT
                taxon_list_item_preferred_name_fkey
            FOREIGN KEY (preferred_name)
            REFERENCES uksi.taxon_list_item (
                taxon_list_item_key
            ) ON DELETE CASCADE;
    "#
        ))
        .await?;

    insert_table::<TaxonDesignationTypeKind>(&conn, transaction).await?;
    insert_table::<TaxonDesignationType>(&conn, transaction).await?;
    insert_table::<TaxonDesignation>(&conn, transaction).await?;
    insert_table::<Aggregate>(&conn, transaction).await?;
    insert_table::<InputGroupMap>(&conn, transaction).await?;

    Ok(())
}

async fn insert_table<T>(
    conn: &Connection,
    transaction: &mut PgTransaction<'static>,
) -> Result<(), UpdateError>
where
    T: Table,
{
    let buffer_size = std::cmp::min(MAX_BUFFER_SIZE, BIND_LIMIT / T::NUM_COLUMNS);
    let mut buffer: Vec<T> = Vec::with_capacity(buffer_size);
    let rows = conn.prepare(T::READ_QUERY)?;

    for row in rows {
        let row = T::from_row(row)?;
        buffer.push(row);

        if buffer.len() >= buffer_size {
            flush_buffer(&mut buffer, transaction).await?;
        }
    }

    flush_buffer(&mut buffer, transaction).await?;

    Ok(())
}

async fn flush_buffer<T>(
    buffer: &mut Vec<T>,
    transaction: &mut PgTransaction<'static>,
) -> Result<(), sqlx::Error>
where
    T: Table,
{
    let mut builder = QueryBuilder::new(T::INSERT_QUERY);
    builder.push_values(&mut *buffer, |builder, item| {
        item.bind_values(builder);
    });

    let query = builder.build();
    transaction.execute(query).await?;

    buffer.clear();

    Ok(())
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    #[sqlx::test(fixtures("../../tests/fixtures.sql"))]
    async fn test_update(_pool: PgPool) {
        // -- TODO --
        // Genereate test db and write proper tests.
        // Last import tested on real UKSI: 2025-11-04.
        //
        // let mut transaction = pool.begin().await.unwrap();
        //
        // update(
        //    PathBuf::from_str("/path/to/UKSI.mdb").unwrap(),
        //    &mut transaction,
        // )
        // .await
        // .unwrap();
        //
        // transaction.commit().await.unwrap();
        todo!();
    }
}
