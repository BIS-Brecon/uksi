use sqlx::{Error, PgTransaction, query};

// Deletes existing data prior to population.
pub(crate) async fn delete(transaction: &mut PgTransaction<'static>) -> Result<(), Error> {
    query!("TRUNCATE TABLE uksi.taxon_fact;")
        .execute(&mut **transaction)
        .await?;
    query!("TRUNCATE TABLE uksi.input_group_map;")
        .execute(&mut **transaction)
        .await?;
    query!("TRUNCATE TABLE uksi.aggregate;")
        .execute(&mut **transaction)
        .await?;
    query!("TRUNCATE TABLE uksi.taxon_designation;")
        .execute(&mut **transaction)
        .await?;
    query!("DELETE FROM uksi.taxon_designation_type;")
        .execute(&mut **transaction)
        .await?;
    query!("DELETE FROM uksi.taxon_designation_type_kind;")
        .execute(&mut **transaction)
        .await?;
    query!("DELETE FROM uksi.taxon_list_item;")
        .execute(&mut **transaction)
        .await?;
    query!("DELETE FROM uksi.taxon_list_type;")
        .execute(&mut **transaction)
        .await?;
    query!("TRUNCATE TABLE uksi.organism;")
        .execute(&mut **transaction)
        .await?;
    query!("TRUNCATE TABLE uksi.nameserver;")
        .execute(&mut **transaction)
        .await?;
    query!("DELETE FROM uksi.taxon_version;")
        .execute(&mut **transaction)
        .await?;
    query!("DELETE FROM uksi.taxon_group_name;")
        .execute(&mut **transaction)
        .await?;
    query!("DELETE FROM uksi.taxon_rank;")
        .execute(&mut **transaction)
        .await?;
    query!("DELETE FROM uksi.taxon;")
        .execute(&mut **transaction)
        .await?;
    query!("DELETE FROM uksi.taxon_name_type;")
        .execute(&mut **transaction)
        .await?;
    query!("DELETE FROM uksi.owner;")
        .execute(&mut **transaction)
        .await?;
    query!("DELETE FROM uksi.individual;")
        .execute(&mut **transaction)
        .await?;

    Ok(())
}
