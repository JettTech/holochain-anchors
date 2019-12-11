use hdk::{
    entry_definition::ValidatingEntryType,
    entry,
    from,
    link,
};
use hdk::holochain_core_types::{
    dna::entry_types::Sharing,
};

use crate::{
    ANCHOR_TYPE,
    ANCHOR_LINK_TYPE,
    Anchor,
};

/// This defines the anchor type that is used to create arbitrary anchors.
/// It must be called from your zome.
pub fn anchor_definition() -> ValidatingEntryType {
    entry!(
        name: ANCHOR_TYPE,
        description: "Anchors are used as the base for links so linked entries can be found with a text search.",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Anchor>| {
            Ok(())
        },
        links: [
            from!(
                ANCHOR_TYPE,
                link_type: ANCHOR_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            )
        ]
    )
}
