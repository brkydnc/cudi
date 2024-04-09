use cust::device::DeviceAttribute::{self, *};

pub const ATTRIBUTE_LIST: [(DeviceAttribute, &str); 89] = [
    (MaxThreadsPerBlock, "Maximum number of threads per block"),
    (MaxBlockDimX, "Maximum x-dimension of a block"),
    (MaxBlockDimY, "Maximum y-dimension of a block"),
    (MaxBlockDimZ, "Maximum z-dimension of a block"),
    (MaxGridDimX, "Maximum x-dimension of a grid"),
    (MaxGridDimY, "Maximum y-dimension of a grid"),
    (MaxGridDimZ, "Maximum z-dimension of a grid"),
    (MaxSharedMemoryPerBlock, "Maximum amount of shared memory available to a thread block in bytes"),
    (TotalConstantMemory, "Memory available on device for constant variables in a kernel in bytes"),
    (WarpSize, "Warp size in threads"),
    (MaxPitch, "Maximum pitch in bytes allowed by the memory copy functions that involve memory regions allocated through cuMemAllocPitch()"),
    (MaxRegistersPerBlock, "Maximum number of 32-bit registers available to a thread block"),
    (ClockRate, "Typical clock frequency in kilohertz"),
    (TextureAlignment, "Alignment requirement for textures"),
    (MultiprocessorCount, "Number of multiprocessors on device."),
    (KernelExecTimeout, "Specifies whether there is a run time limit on kernels"),
    (Integrated, "Device is integrated with host memory"),
    (CanMapHostMemory, "Device can map host memory into CUDA address space"),
    (ComputeMode, "Compute Mode"),
    (MaximumTexture1DWidth, "Maximum 1D texture width"),
    (MaximumTexture2DWidth, "Maximum 2D texture width"),
    (MaximumTexture2DHeight, "Maximum 2D texture height"),
    (MaximumTexture3DWidth, "Maximum 3D texture width"),
    (MaximumTexture3DHeight, "Maximum 3D texture height"),
    (MaximumTexture3DDepth, "Maximum 3D texture depth"),
    (MaximumTexture2DLayeredWidth, "Maximum 2D layered texture width"),
    (MaximumTexture2DLayeredHeight, "Maximum 2D layered texture height"),
    (MaximumTexture2DLayeredLayers, "Maximum layers in a 2D layered texture"),
    (SurfaceAlignment, "Alignment requirement for surfaces"),
    (ConcurrentKernels, "Device can possibly execute multiple kernels concurrently"),
    (EccEnabled, "Device has ECC support enabled"),
    (PciBusId, "PCI bus ID of the device"),
    (PciDeviceId, "PCI device ID of the device"),
    (TccDriver, "Device is using TCC driver model"),
    (MemoryClockRate, "Peak memory clock frequency in kilohertz"),
    (GlobalMemoryBusWidth, "Global memory bus width in bits"),
    (L2CacheSize, "Size of L2 cache in bytes."),
    (MaxThreadsPerMultiprocessor, "Maximum resident threads per multiprocessor"),
    (AsyncEngineCount, "Number of asynchronous engines"),
    (UnifiedAddressing, "Device shares a unified address space with the host"),
    (MaximumTexture1DLayeredWidth, "Maximum 1D layered texture width"),
    (MaximumTexture1DLayeredLayers, "Maximum layers in a 1D layered texture"),
    (MaximumTexture2DGatherWidth, "Maximum 2D texture width if CUDA_ARRAY3D_TEXTURE_GATHER is set"),
    (MaximumTexture2DGatherHeight, "Maximum 2D texture height if CUDA_ARRAY3D_TEXTURE_GATHER is set"),
    (MaximumTexture3DWidthAlternate, "Alternate maximum 3D texture width"),
    (MaximumTexture3DHeightAlternate, "Alternate maximum 3D texture height"),
    (MaximumTexture3DDepthAlternate, "Alternate maximum 3D texture depth"),
    (PciDomainId, "PCI domain ID of the device"),
    (TexturePitchAlignment, "Pitch alignment requirement for textures"),
    (MaximumTextureCubemapWidth, "Maximum cubemap texture width/height"),
    (MaximumTextureCubemapLayeredWidth, "Maximum cubemap layered texture width/height"),
    (MaximumTextureCubemapLayeredLayers, "Maximum layers in a cubemap layered texture"),
    (MaximumSurface1DWidth, "Maximum 1D surface width"),
    (MaximumSurface2DWidth, "Maximum 2D surface width"),
    (MaximumSurface2DHeight, "Maximum 2D surface height"),
    (MaximumSurface3DWidth, "Maximum 3D surface width"),
    (MaximumSurface3DHeight, "Maximum 3D surface height"),
    (MaximumSurface3DDepth, "Maximum 3D surface depth"),
    (MaximumSurface1DLayeredWidth, "Maximum 1D layered surface width"),
    (MaximumSurface1DLayeredLayers, "Maximum layers in a 1D layered surface"),
    (MaximumSurface2DLayeredWidth, "Maximum 2D layered surface width"),
    (MaximumSurface2DLayeredHeight, "Maximum 2D layered surface height"),
    (MaximumSurface2DLayeredLayers, "Maximum layers in a 2D layered surface"),
    (MaximumSurfacecubemapWidth, "Maximum cubemap surface width"),
    (MaximumSurfacecubemapLayeredWidth, "Maximum cubemap layered surface width"),
    (MaximumSurfacecubemapLayeredLayers, "Maximum layers in a cubemap layered surface"),
    (MaximumTexture1DLinearWidth, "Maximum 1D linear texture width"),
    (MaximumTexture2DLinearWidth, "Maximum 2D linear texture width"),
    (MaximumTexture2DLinearHeight, "Maximum 2D linear texture height"),
    (MaximumTexture2DLinearPitch, "Maximum 2D linear texture pitch in bytes"),
    (MaximumTexture2DMipmappedWidth, "Maximum mipmapped 2D texture height"),
    (MaximumTexture2DMipmappedHeight, "Maximum mipmapped 2D texture width"),
    (ComputeCapabilityMajor, "Major compute capability version number"),
    (ComputeCapabilityMinor, "Minor compute capability version number"),
    (MaximumTexture1DMipmappedWidth, "Maximum mipammed 1D texture width"),
    (StreamPrioritiesSupported, "Device supports stream priorities"),
    (GlobalL1CacheSupported, "Device supports caching globals in L1"),
    (LocalL1CacheSupported, "Device supports caching locals in L1"),
    (MaxSharedMemoryPerMultiprocessor, "Maximum shared memory available per multiprocessor in bytes"),
    (MaxRegistersPerMultiprocessor, "Maximum number of 32-bit registers available per multiprocessor"),
    (ManagedMemory, "Device can allocate managed memory on this system"),
    (MultiGpuBoard, "Device is on a multi-GPU board"),
    (MultiGpuBoardGroupId, "Unique ID for a group of devices on the same multi-GPU board"),
    (HostNativeAtomicSupported, "Link between the device and the host supports native atomic operations (this is a placeholder attribute and is not supported on any current hardware)"),
    (SingleToDoublePrecisionPerfRatio, "Ratio of single precision performance (in floating-point operations per second) to double precision performance"),
    (PageableMemoryAccess, "Device supports coherently accessing pageable memory without calling cudaHostRegister on it."),
    (ConcurrentManagedAccess, "Device can coherently access managed memory concurrently with the CPU"),
    (ComputePreemptionSupported, "Device supports compute preemption"),
    (CanUseHostPointerForRegisteredMem, "Device can access host registered memory at the same virtual address as the CPU"),
];