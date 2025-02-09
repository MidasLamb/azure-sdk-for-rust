#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterProperties {
    #[serde(rename = "clusterId", default, skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<cluster_properties::ProvisioningState>,
    #[serde(rename = "isDoubleEncryptionEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_double_encryption_enabled: Option<bool>,
    #[serde(rename = "isAvailabilityZonesEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_availability_zones_enabled: Option<bool>,
    #[serde(rename = "billingType", default, skip_serializing_if = "Option::is_none")]
    pub billing_type: Option<BillingType>,
    #[serde(rename = "keyVaultProperties", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_properties: Option<KeyVaultProperties>,
    #[serde(rename = "lastModifiedDate", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    #[serde(rename = "createdDate", default, skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(rename = "associatedWorkspaces", default, skip_serializing_if = "Vec::is_empty")]
    pub associated_workspaces: Vec<AssociatedWorkspace>,
    #[serde(rename = "capacityReservationProperties", default, skip_serializing_if = "Option::is_none")]
    pub capacity_reservation_properties: Option<CapacityReservationProperties>,
}
pub mod cluster_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        Succeeded,
        Failed,
        Canceled,
        Deleting,
        ProvisioningAccount,
        Updating,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterPatchProperties {
    #[serde(rename = "keyVaultProperties", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_properties: Option<KeyVaultProperties>,
    #[serde(rename = "billingType", default, skip_serializing_if = "Option::is_none")]
    pub billing_type: Option<BillingType>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterPatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterPatchProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ClusterSku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cluster {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ClusterSku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterListResult {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Cluster>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultProperties {
    #[serde(rename = "keyVaultUri", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_uri: Option<String>,
    #[serde(rename = "keyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[serde(rename = "keyVersion", default, skip_serializing_if = "Option::is_none")]
    pub key_version: Option<String>,
    #[serde(rename = "keyRsaSize", default, skip_serializing_if = "Option::is_none")]
    pub key_rsa_size: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum BillingType {
    Cluster,
    Workspaces,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterSku {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<cluster_sku::Name>,
}
pub mod cluster_sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        CapacityReservation,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Identity {
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "type")]
    pub type_: identity::Type,
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
pub mod identity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
        UserAssigned,
        None,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserIdentityProperties {
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssociatedWorkspace {
    #[serde(rename = "workspaceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[serde(rename = "workspaceName", default, skip_serializing_if = "Option::is_none")]
    pub workspace_name: Option<String>,
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "associateDate", default, skip_serializing_if = "Option::is_none")]
    pub associate_date: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CapacityReservationProperties {
    #[serde(rename = "lastSkuUpdate", default, skip_serializing_if = "Option::is_none")]
    pub last_sku_update: Option<String>,
    #[serde(rename = "minCapacity", default, skip_serializing_if = "Option::is_none")]
    pub min_capacity: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TableProperties {
    #[serde(rename = "retentionInDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_in_days: Option<i32>,
    #[serde(rename = "isTroubleshootingAllowed", default, skip_serializing_if = "Option::is_none")]
    pub is_troubleshooting_allowed: Option<bool>,
    #[serde(rename = "isTroubleshootEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_troubleshoot_enabled: Option<bool>,
    #[serde(rename = "lastTroubleshootDate", default, skip_serializing_if = "Option::is_none")]
    pub last_troubleshoot_date: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Table {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TableProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TablesListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Table>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceSku {
    pub name: workspace_sku::Name,
    #[serde(rename = "capacityReservationLevel", default, skip_serializing_if = "Option::is_none")]
    pub capacity_reservation_level: Option<i32>,
    #[serde(rename = "lastSkuUpdate", default, skip_serializing_if = "Option::is_none")]
    pub last_sku_update: Option<String>,
}
pub mod workspace_sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        Free,
        Standard,
        Premium,
        PerNode,
        #[serde(rename = "PerGB2018")]
        PerGb2018,
        Standalone,
        CapacityReservation,
        #[serde(rename = "LACluster")]
        LaCluster,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceCapping {
    #[serde(rename = "dailyQuotaGb", default, skip_serializing_if = "Option::is_none")]
    pub daily_quota_gb: Option<f64>,
    #[serde(rename = "quotaNextResetTime", default, skip_serializing_if = "Option::is_none")]
    pub quota_next_reset_time: Option<String>,
    #[serde(rename = "dataIngestionStatus", default, skip_serializing_if = "Option::is_none")]
    pub data_ingestion_status: Option<workspace_capping::DataIngestionStatus>,
}
pub mod workspace_capping {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DataIngestionStatus {
        RespectQuota,
        ForceOn,
        ForceOff,
        OverQuota,
        SubscriptionSuspended,
        ApproachingQuota,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<workspace_properties::ProvisioningState>,
    #[serde(rename = "customerId", default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<WorkspaceSku>,
    #[serde(rename = "retentionInDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_in_days: Option<i32>,
    #[serde(rename = "workspaceCapping", default, skip_serializing_if = "Option::is_none")]
    pub workspace_capping: Option<WorkspaceCapping>,
    #[serde(rename = "createdDate", default, skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(rename = "modifiedDate", default, skip_serializing_if = "Option::is_none")]
    pub modified_date: Option<String>,
    #[serde(rename = "publicNetworkAccessForIngestion", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access_for_ingestion: Option<PublicNetworkAccessType>,
    #[serde(rename = "publicNetworkAccessForQuery", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access_for_query: Option<PublicNetworkAccessType>,
    #[serde(rename = "forceCmkForQuery", default, skip_serializing_if = "Option::is_none")]
    pub force_cmk_for_query: Option<bool>,
    #[serde(rename = "privateLinkScopedResources", default, skip_serializing_if = "Vec::is_empty")]
    pub private_link_scoped_resources: Vec<PrivateLinkScopedResource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<WorkspaceFeatures>,
}
pub mod workspace_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        Succeeded,
        Failed,
        Canceled,
        Deleting,
        ProvisioningAccount,
        Updating,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceFeatures {
    #[serde(rename = "enableDataExport", default, skip_serializing_if = "Option::is_none")]
    pub enable_data_export: Option<bool>,
    #[serde(rename = "immediatePurgeDataOn30Days", default, skip_serializing_if = "Option::is_none")]
    pub immediate_purge_data_on30_days: Option<bool>,
    #[serde(
        rename = "enableLogAccessUsingOnlyResourcePermissions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_log_access_using_only_resource_permissions: Option<bool>,
    #[serde(rename = "clusterResourceId", default, skip_serializing_if = "Option::is_none")]
    pub cluster_resource_id: Option<String>,
    #[serde(rename = "disableLocalAuth", default, skip_serializing_if = "Option::is_none")]
    pub disable_local_auth: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkScopedResource {
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "scopeId", default, skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Workspace {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspaceProperties>,
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspacePatch {
    #[serde(flatten)]
    pub azure_entity_resource: AzureEntityResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspaceProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Workspace>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PublicNetworkAccessType {
    Enabled,
    Disabled,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorAdditionalInfo {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub location: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureEntityResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
