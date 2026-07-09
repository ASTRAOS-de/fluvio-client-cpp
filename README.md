<h1 align="center">Fluvio Client for C++</h1>
<div align="center">
 <strong>
   Native C++ binding for the Fluvio streaming platform.
 </strong>
</div>
<br />

[![Build](https://github.com/stefanDeveloper/fluvio-client-cpp/actions/workflows/release.yml/badge.svg)](https://github.com/stefanDeveloper/fluvio-client-cpp/actions/workflows/release.yml)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://github.com/stefanDeveloper/fluvio-client-cpp/blob/main/LICENSE)
[![vcpkg](https://img.shields.io/badge/vcpkg-supported-blue.svg)](https://github.com/microsoft/vcpkg)

## Documentation

The client API documentation is written in standard Markdown and generated dynamically into C++ headers. You can find the full API overview in [docs/API.md](docs/API.md).

The public API follows the same object-oriented layout as the original Fluvio client: connect with `Fluvio` or `FluvioAdmin`, then call methods on the returned objects.

## Installation

You can install the client effortlessly without compiling the heavy Rust toolchain by using `vcpkg`.

```bash
vcpkg install fluvio-client-cpp
```

In your `CMakeLists.txt`, simply find the package and link it:

```cmake
find_package(fluvio_client_cpp CONFIG REQUIRED)
target_link_libraries(main PRIVATE ASTRAOS::fluvio_client_cpp)
```

Alternatively, you can build the library from source by fetching it during the CMake configure step and building it as part of your project:

```cmake
include(FetchContent)
FetchContent_Declare(
        fluvio_client_cpp
        GIT_REPOSITORY https://github.com/ASTRAOS-de/fluvio-client-cpp.git
)
FetchContent_MakeAvailable(fluvio_client_cpp)

target_link_libraries(main PRIVATE ASTRAOS::fluvio_client_cpp)
```

# Example Usage

## Creating a Topic

```cpp
#include "fluvio-client-cpp/src/lib.rs.h"

int main() {
    auto admin = FluvioAdmin::connect();
    admin->create_topic("a_topic", 1, 1);
    return 0;
}
```

## Producer

```cpp
#include "fluvio-client-cpp/src/lib.rs.h"
#include "rust/cxx.h"
#include <string>

int main() {
    auto client = Fluvio::connect();
    auto producer = client->topic_producer("my-topic");

    std::string payload = "FOOBAR";
    uint8_t key[] = {};

    auto out = producer->send(
        rust::Slice<const uint8_t>(key, 0), 
        rust::Slice<const uint8_t>(reinterpret_cast<const uint8_t*>(payload.data()), payload.size())
    );
    
    auto meta = out->wait();
    (void)meta;
    producer->flush();
    return 0;
}
```

## Consumer

```cpp
#include "fluvio-client-cpp/src/lib.rs.h"
#include <iostream>

int main() {
    auto client = Fluvio::connect();
    auto stream = client->consumer_stream("my-topic", 0, 0); // Offset::beginning

    auto rec = stream->next();
    auto val = rec->value();
    std::string payload(val.begin(), val.end());
    std::cout << payload << std::endl;

    return 0;
}
```

# Developer Notes

This project uses [CXX](https://cxx.rs) to safely wrap the underlying asynchronous Rust Fluvio crate into native synchronous C++ headers.

For binary distribution, GitHub Actions compiles the Rust library (`libfluvio_client_cpp.a`) and creates a release tarball. The included `vcpkg-port` simply downloads this artifact, bypassing Rust compilation entirely for the end user.

To compile from source locally, ensure you have Rust installed and run:

```bash
cargo build --release
```

To run the integrated test suite locally against your Fluvio cluster, use CTest:

```bash
cmake -B build
cmake --build build
cd build
ctest --output-on-failure
cd ..
```
