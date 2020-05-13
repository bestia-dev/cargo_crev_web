//! duration_mod

use chrono::prelude::*;
#[allow(unused_imports)]
use ansi_term::Colour::{Red,Green,Yellow,Blue};

pub fn start_ns() -> i64 {
    let now = Utc::now();
    now.timestamp_nanos()
}

pub fn eprint_duration_ns(name:&str,start_ns:i64)-> i64{
    let now_ns = Utc::now().timestamp_nanos();
    let duration_ns = now_ns - start_ns;
    let duration_ns=duration_ns.to_string();
    let duration_ns=
    if duration_ns.len()==4{
        format!("          {}.{}", &duration_ns[0..1],&duration_ns[1..4])
    } else if duration_ns.len()== 5{
        format!("         {}.{}", &duration_ns[0..2],&duration_ns[2..5])
    } else if duration_ns.len()==6{
        format!("        {}.{}", &duration_ns[0..3],&duration_ns[3..6])
    } else if duration_ns.len()==7{
        format!("      {}.{}.{}", &duration_ns[0..1],&duration_ns[1..4], &duration_ns[4..7])
    } else if duration_ns.len()==8{
        format!("     {}.{}.{}", &duration_ns[0..2],&duration_ns[2..5], &duration_ns[5..8])
    } else if duration_ns.len()==9{
        format!("    {}.{}.{}", &duration_ns[0..3],&duration_ns[3..6], &duration_ns[6..9])
    } else if duration_ns.len()==10{
        format!("  {}.{}.{}.{}", &duration_ns[0..1],&duration_ns[1..4], &duration_ns[4..7], &duration_ns[7..10])
    } else if duration_ns.len()==11{
        format!(" {}.{}.{}.{}", &duration_ns[0..2],&duration_ns[2..5], &duration_ns[5..8], &duration_ns[8..11])
    } else if duration_ns.len()==12{
        format!("{}.{}.{}.{}", &duration_ns[0..3],&duration_ns[3..6], &duration_ns[6..9], &duration_ns[9..12])
    } else{
        duration_ns
    }
    ;
    eprintln!("{} ns : {}",duration_ns,name);
    // return
    now_ns
}
