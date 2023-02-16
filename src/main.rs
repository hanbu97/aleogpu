use rust_gpu_tools::{cuda, program_closures, Device, GPUError, Program};
use snarkvm::{create_scalar_bases, prelude::TestRng, Fr, G1Affine};

// use crate::test_data::rand::TestRng;

pub mod test_data;
pub use snarkvm;
pub mod opencl;

/// Returns a `Program` that runs on CUDA.
// fn cuda(device: &Device) -> Program {
//     // The kernel was compiled with:
//     // nvcc -fatbin -gencode=arch=compute_52,code=sm_52 -gencode=arch=compute_60,code=sm_60 -gencode=arch=compute_61,code=sm_61 -gencode=arch=compute_70,code=sm_70 -gencode=arch=compute_75,code=sm_75 -gencode=arch=compute_75,code=compute_75 --x cu add.cl
//     let cuda_kernel = include_bytes!("./add.fatbin");
//     let cuda_device = device.cuda_device().unwrap();
//     let cuda_program = cuda::Program::from_bytes(cuda_device, cuda_kernel).unwrap();
//     Program::Cuda(cuda_program)
// }

/// Returns a `Program` that runs on OpenCL.
// fn opencl(device: &Device) -> Program {
//     let opencl_kernel = include_str!("./add.cl");
//     let opencl_device = device.opencl_device().unwrap();
//     let opencl_program = opencl::Program::from_opencl(opencl_device, opencl_kernel).unwrap();
//     Program::Opencl(opencl_program)
// }

pub fn main() {
    let mut rng = TestRng::default();
    let test_size = 1000;
    let (bases, scalars) = create_scalar_bases::<G1Affine, Fr>(&mut rng, test_size);

    let cpu = snarkvm::cpu::msm(bases.as_slice(), scalars.as_slice());
    // let cuda = snarkvm::cuda::msm_cuda(bases.as_slice(), scalars.as_slice()).unwrap();

    let opencl = opencl::msm_opencl(bases.as_slice(), scalars.as_slice()).unwrap();

    assert_eq!(cpu, opencl);
    dbg!("success");

    // This is the core. Here we write the interaction with the GPU independent of whether it is
    // CUDA or OpenCL.
    // let closures = program_closures!(|program, _args| -> Result<Vec<u32>, GPUError> {
    //     // Make sure the input data has the same length.
    //     assert_eq!(aa.len(), bb.len());
    //     let length = aa.len();

    //     // Copy the data to the GPU.
    //     let aa_buffer = program.create_buffer_from_slice(&aa)?;
    //     let bb_buffer = program.create_buffer_from_slice(&bb)?;

    //     // The result buffer has the same length as the input buffers.
    //     let result_buffer = unsafe { program.create_buffer::<u32>(length)? };

    //     // Get the kernel.
    //     let kernel = program.create_kernel("add", 1, 1)?;

    //     // Execute the kernel.
    //     kernel
    //         .arg(&(length as u32))
    //         .arg(&aa_buffer)
    //         .arg(&bb_buffer)
    //         .arg(&result_buffer)
    //         .run()?;

    //     // Get the resulting data.
    //     let mut result = vec![0u32; length];
    //     program.read_into_buffer(&result_buffer, &mut result)?;

    //     Ok(result)
    // });

    // // Get the first available device.
    // let device = *Device::all().first().unwrap();

    // // First we run it on CUDA.
    // // let cuda_program = cuda(device);
    // // let cuda_result = cuda_program.run(closures, ()).unwrap();
    // // assert_eq!(cuda_result, [6, 8, 10, 12]);
    // // println!("CUDA result: {:?}", cuda_result);
    // // unsafe {

    // // }

    // // Then we run it on OpenCL.
    // dbg!(&device);
    // let opencl_program = opencl(device);
    // let opencl_result = opencl_program.run(closures, ()).unwrap();
    // assert_eq!(opencl_result, [6, 8, 10, 12]);
    // println!("OpenCL result: {:?}", opencl_result);
}
