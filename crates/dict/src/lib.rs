// This file contains the main implementation for the dictionary library, which handles dictionary-related functionalities. 

pub mod format;
pub mod rime_loader;
pub mod user_dict;

pub use rime_loader::{
    RimeDictLoader, 
    DictEntry, 
    DictSource, 
    SourcedEntry,
    DictLoadSummary,
    DictStats,
};
pub use user_dict::{
    UserDict,
    WordRecord,
    UserDictStats,
};