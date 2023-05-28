use crate::{api::{Central, Peripheral}, Error, Result};
use tokio::time::{self, Duration};

// Returns the first peripheral that matches the given name.
pub async fn find_peripheral<T: Central>(adapter: &T, device_name: &str) -> Result<T::Peripheral> {
    for peripheral in adapter.peripherals().await? {
        let properties = peripheral.properties().await?;
        if let None = properties {
            continue;
        }
        let properties = properties.unwrap();
        if let Some(local_name) = &properties.local_name {
            if local_name.contains(device_name) {
                return Ok(peripheral);
            }
        }
    }
    Err(Error::DeviceNotFound)
}

// Returns all peripherals that match the given name.
pub async fn find_peripherals<T: Central>(adapter: &T, device_name: &str) -> Result<Vec<T::Peripheral>> {
    let mut matching_peripherals = Vec::new();
    for peripheral in adapter.peripherals().await? {
        let properties = peripheral.properties().await?;
        if let None = properties {
            continue;
        }
        let properties = properties.unwrap();
        if let Some(local_name) = &properties.local_name {
            if local_name.contains(device_name) {
                matching_peripherals.push(peripheral);
            }
        }
    }
    Ok(matching_peripherals)
}

// Continuously looks for a peripheral that matches the given name, checking every `interval` for a total duration of `total_duration`.
use std::pin::Pin;
use futures::Future;

pub async fn find_peripheral_with_timeout<T: Central>(adapter: &T, device_name: &str, interval: Duration, total_duration: Duration) -> Result<T::Peripheral> {
    let mut interval_timer = time::interval(interval);
    
    let search_future: Pin<Box<dyn Future<Output = Result<T::Peripheral>>>> = Box::pin(async {
        loop {
            interval_timer.tick().await;
            match find_peripheral(adapter, device_name).await {
                Ok(peripheral) => return Ok(peripheral),
                Err(_) => continue,
            }
        }
    });

    let search_result = time::timeout(total_duration, search_future).await;

    match search_result {
        Ok(Ok(peripheral)) => Ok(peripheral),
        Ok(Err(_)) => Err(Error::DeviceNotFound),
        Err(_) => Err(Error::TimedOut(total_duration)),
    }
}

