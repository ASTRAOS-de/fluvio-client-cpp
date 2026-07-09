#include <fluvio_client_cpp/fluvio_client_cpp.hpp>
#include <iostream>

int main() {
    try {
        std::cout << "Test: Connecting to Fluvio..." << std::endl;
        auto client = Fluvio::connect();

        std::cout << "Test: Creating stream for 'test-topic' partition 0..." << std::endl;
        auto stream = client->consumer_stream("test-topic", 0, 0); // Offset::beginning()
        
        std::cout << "Test: Fetching one record..." << std::endl;
        auto rec = stream->next();

        auto val = rec->value();
        std::cout << "Fetched record value of size: " << val.size() << std::endl;

        std::cout << "Consumer Test Passed!" << std::endl;
    } catch (const std::exception& e) {
        std::cerr << "Test Failed: " << e.what() << std::endl;
        return 1;
    }
    return 0;
}
