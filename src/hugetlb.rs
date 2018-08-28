/* Hugetlb controller */
use std::path::PathBuf;
use std::fs::File;
use std::io::{Write, Read};

use {HugePageResources, Controllers, Controller, Resources, ControllIdentifier, Subsystem};

#[derive(Debug, Clone)]
pub struct HugeTlbController {
    base: PathBuf,
    path: PathBuf,
}

impl Controller for HugeTlbController {
    fn control_type(self: &Self) -> Controllers { Controllers::HugeTlb }
    fn get_path<'a>(self: &'a Self) -> &'a PathBuf { &self.path }
    fn get_path_mut<'a>(self: &'a mut Self) -> &'a mut PathBuf { &mut self.path }
    fn get_base<'a>(self: &'a Self) -> &'a PathBuf { &self.base }

    fn apply(self: &Self, res: &Resources) {
        /* get the resources that apply to this controller */
        let res: &HugePageResources = &res.hugepages;

        if res.update_values {
            for i in &res.limits {
                self.set_limit_in_bytes(&i.size, i.limit);
            }
        }
    }
}

impl ControllIdentifier for HugeTlbController {
    fn controller_type() -> Controllers {
        Controllers::HugeTlb
    }
}

impl<'a> From<&'a Subsystem> for &'a HugeTlbController {
    fn from(sub: &'a Subsystem) -> &'a HugeTlbController {
        unsafe {
            match sub {
                Subsystem::HugeTlb(c) => c,
                _ => {
                    assert_eq!(1, 0);
                    ::std::mem::uninitialized()
                },
            }
        }
    }
}

fn read_u64_from(mut file: File) -> Option<u64> {
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string.trim().parse().ok()
}

impl HugeTlbController {
    pub fn new(oroot: PathBuf) -> Self {
        let mut root = oroot;
        root.push(Self::controller_type().to_string());
        Self {
            base: root.clone(),
            path: root,
        }
    }
    pub fn size_supported(self: &Self, _hugetlb_size: String) -> bool {
        /* TODO */
        true
    }

    pub fn failcnt(self: &Self, hugetlb_size: &String) -> Option<u64> {
        self.open_path(&format!("hugetlb.{}.failcnt", hugetlb_size), false)
            .and_then(read_u64_from)
    }

    pub fn limit_in_bytes(self: &Self, hugetlb_size: &String) -> Option<u64> {
        self.open_path(&format!("hugetlb.{}.limit_in_bytes", hugetlb_size), false)
            .and_then(read_u64_from)
    }

    pub fn usage_in_bytes(self: &Self, hugetlb_size: &String) -> Option<u64> {
        self.open_path(&format!("hugetlb.{}.usage_in_bytes", hugetlb_size), false)
            .and_then(read_u64_from)
    }
    pub fn max_usage_in_bytes(self: &Self, hugetlb_size: &String) -> Option<u64> {
        self.open_path(&format!("hugetlb.{}.max_usage_in_bytes", hugetlb_size), false)
            .and_then(read_u64_from)
    }

    pub fn set_limit_in_bytes(self: &Self, hugetlb_size: &String, limit: u64) {
        self.open_path(&format!("hugetlb.{}.limit_in_bytes", hugetlb_size), false)
            .and_then(|mut file| {
                file.write_all(limit.to_string().as_ref()).ok()
            });
    }
}