#[link(name = "urlparse",
       vers = "0.1-pre",
       uuid = "11edce46-6e7c-46f8-9bba-586d7b1a3581",
       url = "")];

#[comment = "URL Parser"];
#[license = "Apache License"];
#[crate_type = "lib"];

#[feature(macro_rules, globs, asm, managed_boxes)];

extern mod extra;

pub mod urlparse;
