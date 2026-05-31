#include "fluvio-client-cpp/src/lib.rs.h"
#include "rust/cxx.h"
#include <iostream>

int main() {
    try {
        std::cout << "Test: Connecting to admin..." << std::endl;
        auto admin = FluvioAdmin::connect();
        
        std::cout << "Test: Creating topic 'test-topic' (ignoring if it exists)..." << std::endl;
        try {
            admin->create_topic("test-topic", 1, 1);
        } catch (const std::exception& e) {
            std::cout << "Topic might already exist: " << e.what() << std::endl;
        }

        std::cout << "Test: Connecting to Fluvio..." << std::endl;
        auto client = Fluvio::connect();
        std::cout << "Test: Connecting successful." << std::endl;

        std::cout << "Test: Creating producer for 'test-topic'..." << std::endl;
        auto producer = client->topic_producer("test-topic");

        std::cout << "Test: Sending record..." << std::endl;
        uint8_t key[] = {'t', 'e', 's', 't'};
        uint8_t val[] = {'1', '2', '3'};
        auto out = producer->send(
            rust::Slice<const uint8_t>(key, sizeof(key)), 
            rust::Slice<const uint8_t>(val, sizeof(val))
        );

        std::cout << "Test: Waiting for record confirmation..." << std::endl;
        auto meta = out->wait();
        
        producer->flush();
        std::cout << "Producer Test Passed!" << std::endl;
    } catch (const std::exception& e) {
        std::cerr << "Test Failed: " << e.what() << std::endl;
        return 1;
    }
    return 0;
}
