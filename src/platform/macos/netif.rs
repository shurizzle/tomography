#![allow(non_camel_case_types)]

use crate::types::{Error, Result};

use crate::types::network::{NetworkInterface, Type};

use std::fmt::{self, Debug, Formatter};

use core_foundation::array::{CFArray, CFArrayRef};
use core_foundation::base::TCFType;
use core_foundation::string::{CFString, CFStringRef};

use std::os::raw::{c_char, c_int, c_long, c_uchar, c_uint, c_void};

use std::ffi::CStr;

use std::collections::HashMap;

use ifstructs::ifreq;

use ioctl_rs::SIOCGIFFLAGS;

use libc::{
    close, ioctl, socket, sysctl, CTL_NET, IFF_UP, IPPROTO_IP, PF_INET6, PF_LINK, SOCK_DGRAM,
};

use super::wifi::WiFi;

const IFNAMSIZ: usize = 16;
const NETLINK_GENERIC: c_int = 0;
const IFMIB_SYSTEM: c_int = 1;
const IFMIB_IFCOUNT: c_int = 1;
const IFMIB_IFALLDATA: c_int = 3;
const IFDATA_GENERAL: c_int = 1;

#[link(name = "SystemConfiguration", kind = "framework")]
extern "C" {
    fn SCNetworkInterfaceCopyAll() -> CFArrayRef;
    fn SCNetworkInterfaceGetBSDName(ptr: *const c_void) -> CFStringRef;
    fn SCNetworkInterfaceGetLocalizedDisplayName(ptr: *const c_void) -> CFStringRef;
    fn SCNetworkInterfaceGetInterfaceType(ptr: *const c_void) -> CFStringRef;
}

type __int32_t = c_int;
type __darwin_suseconds_t = __int32_t;
type __darwin_time_t = c_long;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct timeval {
    tv_sec: __darwin_time_t,
    tv_usec: __darwin_suseconds_t,
}

#[cfg(target_pointer_width = "64")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct timeval32 {
    tv_sec: __int32_t,
    tv_usec: __int32_t,
}

#[cfg(target_pointer_width = "64")]
type IF_DATA_TIMEVAL = timeval32;

#[cfg(target_pointer_width = "32")]
type IF_DATA_TIMEVAL = timeval;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct if_data64 {
    ifi_type: c_uchar,
    ifi_typelen: c_uchar,
    ifi_physical: c_uchar,
    ifi_addrlen: c_uchar,
    ifi_hdrlen: c_uchar,
    ifi_recvquota: c_uchar,
    ifi_xmitquota: c_uchar,
    ifi_unused1: c_uchar,
    ifi_mtu: u32,
    ifi_metric: u32,
    ifi_baudrate: u64,
    ifi_ipackets: u64,
    ifi_ierrors: u64,
    ifi_opackets: u64,
    ifi_oerrors: u64,
    ifi_collisions: u64,
    ifi_ibytes: u64,
    ifi_obytes: u64,
    ifi_imcasts: u64,
    ifi_omcasts: u64,
    ifi_iqdrops: u64,
    ifi_noproto: u64,
    ifi_recvtiming: u32,
    ifi_xmittiming: u32,
    ifi_lastchange: IF_DATA_TIMEVAL,
}

#[repr(packed)]
#[derive(Debug, Clone, Copy)]
struct ifmibdata {
    ifmd_name: [c_char; IFNAMSIZ],
    ifmd_pcount: c_uint,
    ifmd_flags: c_uint,
    ifmd_snd_len: c_uint,
    ifmd_snd_maxlen: c_uint,
    ifmd_snd_drops: c_uint,
    ifmd_filler: [c_uint; 4],
    ifmd_data: if_data64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct if_traffic_class {
    ifi_ibepackets: u64,
    ifi_ibebytes: u64,
    ifi_obepackets: u64,
    ifi_obebytes: u64,
    ifi_ibkpackets: u64,
    ifi_ibkbytes: u64,
    ifi_obkpackets: u64,
    ifi_obkbytes: u64,
    ifi_ivipackets: u64,
    ifi_ivibytes: u64,
    ifi_ovipackets: u64,
    ifi_ovibytes: u64,
    ifi_ivopackets: u64,
    ifi_ivobytes: u64,
    ifi_ovopackets: u64,
    ifi_ovobytes: u64,
    ifi_ipvpackets: u64,
    ifi_ipvbytes: u64,
    ifi_opvpackets: u64,
    ifi_opvbytes: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct if_data_extended {
    ifi_alignerrs: u64,
    ifi_dt_bytes: u64,
    ifi_fpackets: u64,
    ifi_fbytes: u64,
    reserved: [u64; 12],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct if_packet_stats {
    ifi_tcp_badformat: u64,
    ifi_tcp_unspecv6: u64,
    ifi_tcp_synfin: u64,
    ifi_tcp_badformatipsec: u64,
    ifi_tcp_noconnnolist: u64,
    ifi_tcp_noconnlist: u64,
    ifi_tcp_listbadsyn: u64,
    ifi_tcp_icmp6unreach: u64,
    ifi_tcp_deprecate6: u64,
    ifi_tcp_rstinsynrcv: u64,
    ifi_tcp_ooopacket: u64,
    ifi_tcp_dospacket: u64,
    ifi_tcp_cleanup: u64,
    ifi_tcp_synwindow: u64,
    reserved: [u64; 6],
    ifi_udp_port_unreach: u64,
    ifi_udp_faithprefix: u64,
    ifi_udp_port0: u64,
    ifi_udp_badlength: u64,
    ifi_udp_badchksum: u64,
    ifi_udp_badmcast: u64,
    ifi_udp_cleanup: u64,
    ifi_udp_badipsec: u64,
    _reserved: [u64; 4],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct if_rxpoll_stats {
    ifi_poll_off_req: u32,
    ifi_poll_off_err: u32,
    ifi_poll_on_req: u32,
    ifi_poll_on_err: u32,
    ifi_poll_wakeups_avg: u32,
    ifi_poll_wakeups_lowat: u32,
    ifi_poll_wakeups_hiwat: u32,
    ifi_poll_packets: u64,
    ifi_poll_packets_avg: u32,
    ifi_poll_packets_min: u32,
    ifi_poll_packets_max: u32,
    ifi_poll_packets_lowat: u32,
    ifi_poll_packets_hiwat: u32,
    ifi_poll_bytes: u64,
    ifi_poll_bytes_avg: u32,
    ifi_poll_bytes_min: u32,
    ifi_poll_bytes_max: u32,
    ifi_poll_bytes_lowat: u32,
    ifi_poll_bytes_hiwat: u32,
    ifi_poll_packets_limit: u32,
    ifi_poll_interval_time: u64,
}

#[derive(Clone)]
struct NetIf {
    ptr: *const c_void,
}

impl NetIf {
    pub fn bsd_name(&self) -> Option<String> {
        unsafe {
            let res = SCNetworkInterfaceGetBSDName(self.ptr);

            if res.is_null() {
                None
            } else {
                Some(CFString::wrap_under_get_rule(res).to_string())
            }
        }
    }

    pub fn display_name(&self) -> Option<String> {
        unsafe {
            let res = SCNetworkInterfaceGetLocalizedDisplayName(self.ptr);

            if res.is_null() {
                None
            } else {
                Some(CFString::wrap_under_get_rule(res).to_string())
            }
        }
    }

    pub fn interface_type(&self) -> Option<String> {
        unsafe {
            let res = SCNetworkInterfaceGetInterfaceType(self.ptr);

            if res.is_null() {
                None
            } else {
                Some(CFString::wrap_under_get_rule(res).to_string())
            }
        }
    }

    pub fn is_up(&self) -> bool {
        match self.bsd_name() {
            None => false,
            Some(name) => {
                let sock = unsafe { socket(PF_INET6, SOCK_DGRAM, IPPROTO_IP) };
                let ifr = ifreq::from_name(&name).unwrap();
                unsafe {
                    ioctl(sock, SIOCGIFFLAGS, &ifr);
                    close(sock);
                }
                (ifr.get_flags() as i32 & IFF_UP) != 0
            }
        }
    }
}

impl Debug for NetIf {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("NetIf")
            .field("bsd_name", &self.bsd_name())
            .field("display_name", &self.display_name())
            .field("interface_type", &self.interface_type())
            .finish()
    }
}

fn if_count() -> Result<usize> {
    let mut name: [c_int; 5] = [0; 5];
    let mut ifcount: c_uint = 0;
    let mut len: usize = ::std::mem::size_of::<c_int>();

    name[0] = CTL_NET;
    name[1] = PF_LINK;
    name[2] = NETLINK_GENERIC;
    name[3] = IFMIB_SYSTEM;
    name[4] = IFMIB_IFCOUNT;

    unsafe {
        if sysctl(
            name.as_mut_ptr(),
            5,
            &mut ifcount as *mut c_uint as *mut c_void,
            &mut len,
            ::std::ptr::null_mut(),
            0,
        ) == -1
        {
            Err(Error::from_errno())
        } else {
            Ok(ifcount as usize)
        }
    }
}

fn _get_ifmibdata(size: usize) -> Result<HashMap<String, ifmibdata>> {
    let mut name: [c_int; 6] = [0; 6];
    let mut ifmd: Vec<ifmibdata> = vec![unsafe { ::std::mem::zeroed() }; size];
    let mut len: usize = ::std::mem::size_of::<ifmibdata>() * size;

    name[0] = CTL_NET;
    name[1] = PF_LINK;
    name[2] = NETLINK_GENERIC;
    name[3] = IFMIB_IFALLDATA;
    name[4] = 0;
    name[5] = IFDATA_GENERAL;

    unsafe {
        if sysctl(
            name.as_mut_ptr(),
            6,
            ifmd.as_mut_ptr() as *mut c_void,
            &mut len,
            ::std::ptr::null_mut(),
            0,
        ) == -1
        {
            Err(Error::from_errno())
        } else {
            let mut hm: HashMap<String, ifmibdata> = HashMap::with_capacity(size);
            for i in 0..size {
                let v = ifmd[i];
                let k = CStr::from_ptr(v.ifmd_name.as_ptr())
                    .to_str()
                    .unwrap()
                    .to_string();
                hm.insert(k, v);
            }
            Ok(hm)
        }
    }
}

fn get_ifmibdata() -> Result<HashMap<String, ifmibdata>> {
    _get_ifmibdata(if_count()?)
}

pub fn all() -> Result<Vec<NetworkInterface>> {
    let ifmib = get_ifmibdata()?;

    unsafe {
        Ok(
            CFArray::<*const c_void>::wrap_under_get_rule(SCNetworkInterfaceCopyAll())
                .iter()
                .map(|ptr| NetIf { ptr: *ptr })
                .filter_map(|netif| {
                    if netif.bsd_name().is_some() && netif.interface_type().is_some() {
                        let t = netif.interface_type().unwrap();
                        if t == "Ethernet" {
                            let name = netif.bsd_name().unwrap();
                            let up = ifmib.get(&name)?.ifmd_data.ifi_obytes;
                            let down = ifmib.get(&name)?.ifmd_data.ifi_ibytes;

                            Some(NetworkInterface {
                                name,
                                display_name: netif.display_name(),
                                itype: Type::Wired,
                                is_up: netif.is_up(),
                                bssid: None,
                                ssid: None,
                                up: up,
                                down: down,
                            })
                        } else if t == "IEEE80211" {
                            let name = netif.bsd_name().unwrap();
                            let wifi = WiFi::new(&name).ok()?;
                            let ifm = ifmib.get(&name)?;
                            let up = ifm.ifmd_data.ifi_obytes;
                            let down = ifm.ifmd_data.ifi_ibytes;

                            let (is_up, bssid, ssid) = if netif.is_up() {
                                let is_up = wifi.on_power().ok()?;
                                if is_up {
                                    (true, wifi.bssid().ok(), wifi.ssid().ok())
                                } else {
                                    (false, None, None)
                                }
                            } else {
                                (false, None, None)
                            };

                            Some(NetworkInterface {
                                name: name,
                                display_name: netif.display_name(),
                                itype: Type::WiFi,
                                is_up: is_up,
                                bssid: bssid,
                                ssid: ssid,
                                up: up,
                                down: down,
                            })
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>(),
        )
    }
}
