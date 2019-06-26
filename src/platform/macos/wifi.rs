use core_foundation::base::TCFType;
use core_foundation::data::{CFData, CFDataRef};
use core_foundation::string::{CFString, CFStringRef};
use libc::{c_int, c_void, strerror, strlen};
use std::{fmt, slice, str};

// Thanks to http://newosxbook.com/articles/11208ellpA.html and
// http://newosxbook.com/articles/11208ellpA-II.html

type Apple80211Ref = *const c_void;

#[link(name = "Apple80211", kind = "framework")]
extern "C" {
    fn Apple80211Open(ptr: *const Apple80211Ref) -> i32;
    fn Apple80211BindToInterface(handle: Apple80211Ref, interface: CFStringRef) -> i32;
    fn Apple80211GetPower(handle: Apple80211Ref, power: *const u32) -> i32;
    fn Apple80211CopyValue(
        handle: Apple80211Ref,
        field: c_int,
        dictCanBeLeftNULL: *const c_void,
        outValue: *const c_void,
    ) -> i32;
    fn Apple80211Close(x: Apple80211Ref) -> i32;
}

fn get_error_string(errno: i32) -> &'static str {
    match errno {
        -3931 => "Error",
        -3930 => "Operation not permitted",
        -3929 => "IPC error",
        -3928 => "Ref not bound",
        -3927 => "Station does not support PCO transition time required by AP",
        -3926 => "Associating station does not support required 802.11n features",
        -3925 => "Supplicant timeout",
        -3924 => "Invalid PMK",
        -3923 => "Cipher suite rejected",
        -3922 => "Invalid RSN capabilities",
        -3921 => "Unsupported RSN version",
        -3920 => "Invalid AKMP",
        -3919 => "Invalid pairwise cipher",
        -3918 => "Invalid group cipher",
        -3917 => "Invalid information element",
        -3916 => "DSSS/OFDM Unsupported",
        -3915 => "Short slot unsupported",
        -3914 => "Unsupported rate set",
        -3913 => "AP full",
        -3912 => "Challenge failure",
        -3911 => "Invalid authentication sequence number",
        -3910 => "Authentication algorithm unsupported",
        -3909 => "Association denied",
        -3908 => "Reassociation denied",
        -3907 => "Unsupported capabilities",
        -3906 => "Unspecified failure",
        -3905 => "Terror -3905 is Terris Format error",
        -3903 => "Operation not supported",
        -3902 => "Unknown error",
        -3901 => "Unable to allocate memory",
        -3900 => "Parameter error",
        _ => unsafe {
            let err = strerror(errno) as *const u8;
            str::from_utf8_unchecked(slice::from_raw_parts(err, strlen(err as *const i8)))
        },
    }
}

pub struct Error(i32);

impl std::error::Error for Error {
    fn description(&self) -> &str {
        get_error_string(self.0)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", get_error_string(self.0))
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Error")
            .field("code", &self.0)
            .field("description", &get_error_string(self.0))
            .finish()
    }
}

fn get_ssid(handle: Apple80211Ref) -> Result<String, Error> {
    let ptr: CFDataRef = std::ptr::null();
    let res = unsafe {
        Apple80211CopyValue(
            handle,
            1,
            std::ptr::null_mut(),
            &ptr as *const CFDataRef as *const c_void,
        )
    };

    if res == 0 {
        Ok(unsafe {
            str::from_utf8_unchecked(CFData::wrap_under_create_rule(ptr).bytes()).to_string()
        })
    } else {
        Err(Error(res))
    }
}

fn get_bssid(handle: Apple80211Ref) -> Result<String, Error> {
    let ptr: CFStringRef = std::ptr::null();
    let res = unsafe {
        Apple80211CopyValue(
            handle,
            9,
            std::ptr::null_mut(),
            &ptr as *const CFStringRef as *const c_void,
        )
    };

    if res == 0 {
        Ok(unsafe { CFString::wrap_under_create_rule(ptr).to_string() })
    } else {
        Err(Error(res))
    }
}

pub struct WiFi(Apple80211Ref);

impl WiFi {
    pub fn new(interface: &str) -> Result<WiFi, Error> {
        let handle: Apple80211Ref = std::ptr::null();

        let res = unsafe { Apple80211Open(&handle) };

        if res == 0 {
            let name = CFString::new(interface);
            let res = unsafe { Apple80211BindToInterface(handle, name.as_concrete_TypeRef()) };

            if res == 0 {
                Ok(WiFi(handle))
            } else {
                unsafe { Apple80211Close(handle) };
                Err(Error(res))
            }
        } else {
            Err(Error(res))
        }
    }

    pub fn on_power(&self) -> Result<bool, Error> {
        let power: u32 = 0;
        let res = unsafe { Apple80211GetPower(self.0, &power) };

        if res == 0 {
            Ok(power != 0)
        } else {
            Err(Error(res))
        }
    }

    pub fn ssid(&self) -> Result<String, Error> {
        get_ssid(self.0)
    }

    pub fn bssid(&self) -> Result<String, Error> {
        get_bssid(self.0)
    }
}

impl Drop for WiFi {
    fn drop(&mut self) {
        unsafe { Apple80211Close(self.0) };
    }
}

impl fmt::Debug for WiFi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("WiFi")
            .field("on_power", &self.on_power())
            .field("bssid", &self.bssid())
            .field("ssid", &self.ssid())
            .finish()
    }
}

#[cfg(test)]
mod test {
    use super::WiFi;

    #[test]
    fn it_works() {
        println!("{:#?}", WiFi::new("en0"));
    }
}
