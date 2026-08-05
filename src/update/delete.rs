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
    query!("TRUNCATE TABLE uksi.taxon_designation_type CASCADE;")
        .execute(&mut **transaction)
        .await?;
    query!("TRUNCATE TABLE uksi.taxon_designation_type_kind CASCADE;")
        .execute(&mut **transaction)
        .await?;
    query!("TRUNCATE TABLE uksi.taxon_list_item CASCADE;")
        .execute(&mut **transaction)
        .await?;
    query!("TRUNCATE TABLE uksi.taxon_list_type CASCADE;")
        .execute(&mut **transaction)
        .await?;
    query!("TRUNCATE TABLE uksi.organism;")
        .execute(&mut **transaction)
        .await?;
    query!("TRUNCATE TABLE uksi.nameserver;")
        .execute(&mut **transaction)
        .await?;
    query!("TRUNCATE TABLE uksi.taxon_version CASCADE;")
        .execute(&mut **transaction)
        .await?;
    query!("TRUNCATE TABLE uksi.taxon_group_name CASCADE;")
        .execute(&mut **transaction)
        .await?;
    query!("TRUNCATE TABLE uksi.taxon_rank CASCADE;")
        .execute(&mut **transaction)
        .await?;
    query!("TRUNCATE TABLE uksi.taxon CASCADE;")
        .execute(&mut **transaction)
        .await?;
    query!("TRUNCATE TABLE uksi.taxon_name_type CASCADE;")
        .execute(&mut **transaction)
        .await?;
    query!("TRUNCATE TABLE uksi.owner CASCADE;")
        .execute(&mut **transaction)
        .await?;
    query!("TRUNCATE TABLE uksi.individual CASCADE;")
        .execute(&mut **transaction)
        .await?;

    Ok(())
}
