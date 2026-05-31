#include "fluvio-client-cpp/src/lib.rs.h"
#include <iostream>

int main() {
    try {
        std::cout << "Test: Creating Consumer Config..." << std::endl;
        auto consumerConfigBuilder = ConsumerConfigBuilder::create()
            ->max_bytes(1024)
            ->disable_continuous(true);

        std::cout << "Test: Creating Producer Config..." << std::endl;
        auto producerConfigBuilder = TopicProducerConfigBuilder::create()
            ->batch_size(1024)
            ->linger(1000);
        
        std::cout << "Config Test Passed!" << std::endl;
    } catch (const std::exception& e) {
        std::cerr << "Test Failed: " << e.what() << std::endl;
        return 1;
    }
    return 0;
}
