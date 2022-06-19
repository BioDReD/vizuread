use cpython::{PyResult, Python, py_module_initializer, py_fn, PyList, PyObject};
use std::fs::File;
use noodles::{bam, sam};
use noodles::core::{Region, Position};
use std::collections::HashMap;
use std::vec::Vec;

struct ReadData {
    
}

// add bindings to the generated python module
// N.B: names: "rust2py" must be the name of the `.so` or `.pyd` file
py_module_initializer!(get_reads, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "get_reads", py_fn!(py, get_reads(path_to_bam : String, region : String, path_to_bai: String)))?;
    Ok(())
});

fn get_bai(path_to_bam : &std::path::PathBuf) -> std::path::PathBuf {
    return std::path::PathBuf::from("-");
}

// logic implemented as a normal rust function
fn _get_reads(
    path_to_bam : std::path::PathBuf, 
    region : String, 
    mut path_to_bai : std::path::PathBuf
    ) -> Vec<PyObject>
{    
    let mut read_list : Vec<ReadData> = Vec::new();
    
    if path_to_bai == std::path::PathBuf::from("-") {
        path_to_bai = get_bai(&path_to_bam);
    }
    
    let mut reader = File::open(path_to_bam).map(bam::Reader::new).unwrap();
    let header: sam::Header = reader.read_header().unwrap().parse().unwrap();
    let reference_sequences = header.reference_sequences();
    let index = bam::bai::read(path_to_bai).unwrap();
    
    let region : Region = region.parse().unwrap();
    let query = reader.query(&reference_sequences, &index, &region).unwrap();
    
    let mut i = 0;
    for result in query {
        let mut record = result.unwrap();
        let cigar = record.cigar_mut().clone();
        let start_pos = record.position();
        let seq = record.sequence_mut().clone();
    }
    
    let mut read_list : Vec<PyObject> = Vec::new(); 
    return read_list;
}

// rust-cpython aware function. All of our python interface could be
// declared in a separate module.
// Note that the py_fn!() macro automatically converts the arguments from
// Python objects to Rust values; and the Rust return value back into a Python object.
fn get_reads(py: Python, path_to_bam : String, region : String, path_to_bai: String) -> PyResult<PyList> {
    let out = _get_reads(std::path::PathBuf::from(path_to_bam) , region, std::path::PathBuf::from(path_to_bai)) ;
    let reads = PyList::new(py, &out);
    Ok(reads)
}