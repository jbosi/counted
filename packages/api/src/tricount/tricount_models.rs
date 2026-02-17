use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TricountApiResponse {
    #[serde(rename = "Response")]
    pub response: Vec<TricountResponseItem>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum TricountResponseItem {
    Registry {
        #[serde(rename = "Registry")]
        registry: TricountRegistry,
    },
    Other(serde_json::Value),
}

#[derive(Deserialize, Debug)]
pub struct TricountRegistry {
    pub title: String,
    pub currency: String,
    pub memberships: Vec<TricountMembershipWrapper>,
    pub all_registry_entry: Vec<TricountEntryWrapper>,
}

#[derive(Deserialize, Debug)]
pub struct TricountMembershipWrapper {
    #[serde(rename = "RegistryMembershipNonUser")]
    pub non_user: Option<TricountMembership>,
}

#[derive(Deserialize, Debug)]
pub struct TricountMembership {
    pub uuid: String,
    pub alias: TricountAlias,
}

#[derive(Deserialize, Debug)]
pub struct TricountAlias {
    pub display_name: String,
}

#[derive(Deserialize, Debug)]
pub struct TricountEntryWrapper {
    #[serde(rename = "RegistryEntry")]
    pub entry: Option<TricountEntry>,
}

#[derive(Deserialize, Debug)]
pub struct TricountEntry {
    pub description: String,
    pub amount: TricountAmount,
    pub membership_owned: TricountMembershipOwned,
    pub allocations: Vec<TricountAllocation>,
    pub created: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct TricountAmount {
    pub value: String,
    pub currency: String,
}

#[derive(Deserialize, Debug)]
pub struct TricountMembershipOwned {
    #[serde(rename = "RegistryMembershipNonUser")]
    pub non_user: Option<TricountMembershipRef>,
}

#[derive(Deserialize, Debug)]
pub struct TricountMembershipRef {
    pub uuid: String,
}

#[derive(Deserialize, Debug)]
pub struct TricountAllocation {
    pub membership: TricountAllocationMembership,
    pub amount: TricountAmount,
}

#[derive(Deserialize, Debug)]
pub struct TricountAllocationMembership {
    #[serde(rename = "RegistryMembershipNonUser")]
    pub non_user: Option<TricountMembershipRef>,
}
