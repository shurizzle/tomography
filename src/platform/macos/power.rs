use crate::types::power::*;

use core_foundation::array::{CFArray, CFArrayRef};
use core_foundation::base::{CFType, CFTypeRef, TCFType};
use core_foundation::boolean::{CFBoolean, CFBooleanRef};
use core_foundation::dictionary::{CFDictionary, CFDictionaryRef};
use core_foundation::number::{CFNumber, CFNumberRef};
use core_foundation::string::{CFString, CFStringRef};

#[link(name = "IOKit", kind = "framework")]
extern "C" {
    fn IOPSCopyPowerSourcesInfo() -> CFTypeRef;
    fn IOPSCopyPowerSourcesList(blob: CFTypeRef) -> CFArrayRef;
    fn IOPSGetPowerSourceDescription(blob: CFTypeRef, ps: CFTypeRef) -> CFDictionaryRef;
    fn IOPSCopyExternalPowerAdapterDetails() -> CFDictionaryRef;
    fn IOPSGetProvidingPowerSourceType(snapshot: CFTypeRef) -> CFStringRef;
}

#[allow(non_upper_case_globals)]
const kIOPMACPowerKey: &str = "AC Power";
#[allow(non_upper_case_globals)]
const kIOPMBatteryPowerKey: &str = "Battery Power";
#[allow(non_upper_case_globals)]
const kIOPMUPSPowerKey: &str = "UPS Power";

#[allow(non_snake_case)]
fn dictionary_to_Battery(dic: CFDictionary<CFString, CFString>) -> Battery {
    let (keys, values) = dic.get_keys_and_values();
    let mut present: Option<bool> = None;
    let mut charged: bool = false;
    let mut state: Option<String> = None;
    let mut current: Option<i64> = None;
    let mut finishing_charge: Option<bool> = None;
    let mut max_capacity: Option<i64> = None;
    let mut design_cycle_count: Option<i64> = None;
    let mut capacity: Option<i64> = None;
    let mut provides_time_remaining: Option<bool> = None;
    let mut charging: Option<bool> = None;
    let mut time_remaining: Option<i64> = None;
    let mut id: Option<i64> = None;
    let mut time_to_charge: Option<i64> = None;
    let mut name: Option<String> = None;
    let mut serial_number: Option<String> = None;
    let mut transport_type: Option<String> = None;
    let mut power_type: Option<String> = None;
    let mut health: Option<String> = None;

    for i in 0..dic.len() {
        unsafe {
            let key: String = CFString::wrap_under_get_rule(keys[i] as CFStringRef).to_string();

            if key == "Is Present" {
                present = Some(CFBoolean::wrap_under_get_rule(values[i] as CFBooleanRef).into());
            } else if key == "Is Charged" {
                charged = CFBoolean::wrap_under_get_rule(values[i] as CFBooleanRef).into();
            } else if key == "Power Source State" {
                state = Some(CFString::wrap_under_get_rule(values[i] as CFStringRef).to_string());
            } else if key == "Current" {
                current = Some(
                    CFNumber::wrap_under_get_rule(values[i] as CFNumberRef)
                        .to_i64()
                        .unwrap(),
                );
            } else if key == "Is Finishing Charge" {
                finishing_charge =
                    Some(CFBoolean::wrap_under_get_rule(values[i] as CFBooleanRef).into());
            } else if key == "Max Capacity" {
                max_capacity = Some(
                    CFNumber::wrap_under_get_rule(values[i] as CFNumberRef)
                        .to_i64()
                        .unwrap(),
                );
            } else if key == "DesignCycleCount" {
                design_cycle_count = Some(
                    CFNumber::wrap_under_get_rule(values[i] as CFNumberRef)
                        .to_i64()
                        .unwrap(),
                );
            } else if key == "Current Capacity" {
                capacity = Some(
                    CFNumber::wrap_under_get_rule(values[i] as CFNumberRef)
                        .to_i64()
                        .unwrap(),
                );
            } else if key == "Battery Provides Time Remaining" {
                provides_time_remaining =
                    Some(CFBoolean::wrap_under_get_rule(values[i] as CFBooleanRef).into());
            } else if key == "Is Charging" {
                charging = Some(CFBoolean::wrap_under_get_rule(values[i] as CFBooleanRef).into());
            } else if key == "Time to Empty" {
                time_remaining = Some(
                    CFNumber::wrap_under_get_rule(values[i] as CFNumberRef)
                        .to_i64()
                        .unwrap(),
                );
            } else if key == "Power Source ID" {
                id = Some(
                    CFNumber::wrap_under_get_rule(values[i] as CFNumberRef)
                        .to_i64()
                        .unwrap(),
                );
            } else if key == "Time to Full Charge" {
                time_to_charge = Some(
                    CFNumber::wrap_under_get_rule(values[i] as CFNumberRef)
                        .to_i64()
                        .unwrap(),
                );
            } else if key == "Name" {
                name = Some(CFString::wrap_under_get_rule(values[i] as CFStringRef).to_string());
            } else if key == "Hardware Serial Number" {
                serial_number =
                    Some(CFString::wrap_under_get_rule(values[i] as CFStringRef).to_string());
            } else if key == "Transport Type" {
                transport_type =
                    Some(CFString::wrap_under_get_rule(values[i] as CFStringRef).to_string());
            } else if key == "Type" {
                power_type =
                    Some(CFString::wrap_under_get_rule(values[i] as CFStringRef).to_string());
            } else if key == "BatteryHealth" {
                health = Some(CFString::wrap_under_get_rule(values[i] as CFStringRef).to_string());
            } else {
                println!("!! {}", key);
            }
        }
    }

    Battery {
        present: present.unwrap(),
        charged: charged,
        state: state.unwrap(),
        current: current.unwrap(),
        finishing_charge: finishing_charge.unwrap_or(false),
        max_capacity: max_capacity.unwrap(),
        design_cycle_count: design_cycle_count.unwrap(),
        capacity: capacity.unwrap(),
        provides_time_remaining: provides_time_remaining.unwrap(),
        charging: charging.unwrap(),
        time_remaining: if provides_time_remaining.unwrap() {
            Some(time_remaining.unwrap())
        } else {
            None
        },
        id: id.unwrap(),
        time_to_charge: time_to_charge.unwrap(),
        name: name.unwrap(),
        serial_number: serial_number.unwrap(),
        transport_type: transport_type.unwrap(),
        power_type: power_type.unwrap(),
        health: health.unwrap(),
    }
}

#[allow(non_snake_case)]
fn dictionary_to_Adapter(dic: CFDictionary<CFString, CFString>) -> Adapter {
    let (keys, values) = dic.get_keys_and_values();
    let mut id: Option<i64> = None;
    let mut serial_number: Option<i64> = None;
    let mut source: Option<i64> = None;
    let mut family: Option<i64> = None;
    let mut watts: Option<i64> = None;
    let mut current: Option<i64> = None;
    let mut voltage: Option<i64> = None;

    for i in 0..dic.len() {
        unsafe {
            let key: String = CFString::wrap_under_get_rule(keys[i] as CFStringRef).to_string();

            if key == "AdapterID" {
                id = Some(
                    CFNumber::wrap_under_get_rule(values[i] as CFNumberRef)
                        .to_i64()
                        .unwrap(),
                );
            } else if key == "SerialNumber" {
                serial_number = Some(
                    CFNumber::wrap_under_get_rule(values[i] as CFNumberRef)
                        .to_i64()
                        .unwrap(),
                );
            } else if key == "Source" {
                source = Some(
                    CFNumber::wrap_under_get_rule(values[i] as CFNumberRef)
                        .to_i64()
                        .unwrap(),
                );
            } else if key == "FamilyCode" {
                family = Some(
                    CFNumber::wrap_under_get_rule(values[i] as CFNumberRef)
                        .to_i64()
                        .unwrap(),
                );
            } else if key == "Watts" {
                watts = Some(
                    CFNumber::wrap_under_get_rule(values[i] as CFNumberRef)
                        .to_i64()
                        .unwrap(),
                );
            } else if key == "Current" {
                current = Some(
                    CFNumber::wrap_under_get_rule(values[i] as CFNumberRef)
                        .to_i64()
                        .unwrap(),
                );
            } else if key == "Voltage" {
                voltage = Some(
                    CFNumber::wrap_under_get_rule(values[i] as CFNumberRef)
                        .to_i64()
                        .unwrap(),
                );
            } else {
                println!("!! {}", key);
            }
        }
    }

    Adapter {
        id: id.unwrap(),
        serial_number: serial_number.unwrap(),
        source: source.unwrap(),
        family: family.unwrap(),
        watts: watts.unwrap(),
        current: current.unwrap(),
        voltage: voltage.unwrap(),
    }
}

fn info() -> Option<CFType> {
    let ptr = unsafe { IOPSCopyPowerSourcesInfo() };
    if ptr.is_null() {
        None
    } else {
        unsafe { Some(CFType::wrap_under_get_rule(ptr)) }
    }
}

fn list(blob: &CFType) -> Option<CFArray<CFType>> {
    let ptr = unsafe { IOPSCopyPowerSourcesList(blob.as_concrete_TypeRef()) };
    if ptr.is_null() {
        None
    } else {
        unsafe { Some(CFArray::wrap_under_get_rule(ptr)) }
    }
}

fn description(blob: &CFType, ps: &CFType) -> Option<Battery> {
    let ptr = unsafe {
        IOPSGetPowerSourceDescription(blob.as_concrete_TypeRef(), ps.as_concrete_TypeRef())
    };

    if ptr.is_null() {
        None
    } else {
        unsafe {
            Some(dictionary_to_Battery(CFDictionary::wrap_under_get_rule(
                ptr,
            )))
        }
    }
}

fn sources(i: &CFType) -> Option<Vec<Battery>> {
    match list(i) {
        None => None,
        Some(list) => Some(
            list.iter()
                .map(|x| description(i, &x))
                .filter(|x| x.is_some())
                .map(|x| x.unwrap())
                .collect(),
        ),
    }
}

fn power_type(snapshot: &CFType) -> Type {
    let t = unsafe {
        CFString::wrap_under_get_rule(IOPSGetProvidingPowerSourceType(
            snapshot.as_concrete_TypeRef(),
        ))
        .to_string()
    };

    if t == kIOPMACPowerKey {
        Type::AC
    } else if t == kIOPMBatteryPowerKey {
        Type::BATTERY
    } else if t == kIOPMUPSPowerKey {
        Type::UPS
    } else {
        panic!("Unknown type")
    }
}

fn adapter() -> Option<Adapter> {
    let ptr = unsafe { IOPSCopyExternalPowerAdapterDetails() };
    if ptr.is_null() {
        None
    } else {
        unsafe {
            Some(dictionary_to_Adapter(CFDictionary::wrap_under_get_rule(
                ptr,
            )))
        }
    }
}

pub fn all() -> PowerSources {
    match info() {
        None => PowerSources {
            sources: None,
            power_type: power_type(&(unsafe { CFType::wrap_under_get_rule(0 as CFTypeRef) })),
            adapter: None,
        },
        Some(i) => PowerSources {
            sources: sources(&i),
            power_type: power_type(&i),
            adapter: adapter(),
        },
    }
}
