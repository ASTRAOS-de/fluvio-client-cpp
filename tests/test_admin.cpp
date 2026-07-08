#include <fluvio_client_cpp/fluvio_client_cpp.hpp>
#include <iostream>

int main() {
    try {
        std::cout << "Test: Connecting to admin..." << std::endl;
        auto admin = FluvioAdmin::connect();
        
        std::cout << "Test: Creating topic 'admin-test-topic'..." << std::endl;
        try {
            admin->create_topic("admin-test-topic", 1, 1);
        } catch (...) {}
        
        std::cout << "Test: Deleting topic 'admin-test-topic'..." << std::endl;
        admin->delete_topic("admin-test-topic");

        std::cout << "Admin Test Passed!" << std::endl;
    } catch (const std::exception& e) {
        std::cerr << "Test Failed: " << e.what() << std::endl;
        return 1;
    }
    return 0;
}
