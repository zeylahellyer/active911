//! API model definitions for working with the Active911 "API".

use serde::{Deserialize, Serialize};

/// Information about a fire department agency.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Agency {
    /// Name of the agency, such as "Boone FD".
    pub name: String,
    /// Timezone the agency is in, such as "America/New_York".
    pub timezone: String,
    /// Unix timestamp at the time of retrieval.
    pub timestamp: u64,
    /// Latitude of where the agency is located.
    #[serde(rename = "lat")]
    pub latitude: f64,
    /// Longitude of where the agency is located.
    #[serde(rename = "lon")]
    pub longitude: f64,
}

/// Response from retrieving an agency's recent alarms.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AlarmResponse {
    /// Information about the fire department agency.
    pub agency: Agency,
    /// List of the most recent alarms to occur.
    pub alarms: [Alarm; 5],
    /// Message with additional information if the result is a failure.
    ///
    /// In the case of "success" [`result`] value this will usually be an empty
    /// string ("").
    ///
    /// [`result`]: Self::result
    pub message: String,
    /// Response result type, such as "success".
    pub result: String,
}

/// Information about an individual alarm event.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Alarm {
    /// Street number and name of the incident, such as "904 FOO ST".
    pub address: String,
    /// Name of the city, such as "BOONE".
    pub city: String,
    /// Description such as "FIRE ALARM".
    pub description: String,
    /// Generic title such as "Fire".
    pub generic_title: String,
    /// Unique ID of the alarm.
    pub id: u64,
    /// Latitude of where the incident took place.
    #[serde(rename = "lat")]
    pub latitude: f64,
    /// Longitude of where the incident took place.
    #[serde(rename = "lon")]
    pub longitude: f64,
    /// Place of where the incident occurred.
    ///
    /// This is usually empty.
    pub place: String,
    /// Pretty format of when the alarm happened, such as "33 min ago".
    pub pretty_date: String,
    /// Timestamp of when the alarm happened as a Unix timestamp.
    pub stamp: u64,
    /// Two letter code of the state, such as "NC".
    pub state: String,
}

#[cfg(test)]
mod tests {
    use super::{Agency, Alarm, AlarmResponse};
    use serde::{Deserialize, Serialize};
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(
        Agency: Clone,
        Debug,
        Deserialize<'static>,
        Send,
        Serialize,
        Sync
    );
    assert_impl_all!(
        Alarm: Clone,
        Debug,
        Deserialize<'static>,
        Send,
        Serialize,
        Sync
    );
    assert_impl_all!(
        AlarmResponse: Clone,
        Debug,
        Deserialize<'static>,
        Send,
        Serialize,
        Sync
    );
}
