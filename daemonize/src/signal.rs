// use crate::error::{get_err, Error, Result};

// /// Can't find it in libc, this value has been taken from nyx::sys::signal, but it's the same as in
// /// signal.h
// const NSIG: libc::c_int = 32;

// unsafe fn reset_sig_handlers() -> Result<()> {
//     for i in 0..NSIG {
//         eprintln!("signal num : {i}");
//         get_err(libc::signal(i, libc::SIG_DFL), Error::SignalSetting)?;
//     }
//     Ok(())
// }

// unsafe fn reset_sig_mask() -> Result<()> {
//     let null = std::ptr::null_mut();
//     let set = std::ptr::null_mut();
//     get_err(libc::sigemptyset(set), Error::SetSig)?;
//     get_err(
//         libc::sigprocmask(libc::SIG_SETMASK, set, null),
//         Error::SigMask,
//     )?;
//     Ok(())
// }
