#[cfg(feature = "mi-alloc")]
use mimalloc::MiMalloc;

#[cfg(feature = "mi-alloc")]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[cfg(not(any(feature = "mi-alloc", feature = "std-alloc")))]
compile_error!(
	"Either 'mi-alloc' or 'std-alloc' feature must be enabled"
);
