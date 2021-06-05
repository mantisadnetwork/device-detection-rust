#include "device-detection-cxx/src/hash/EngineHash.hpp"
#include "rust/cxx.h"
#include "shim.hpp"

using namespace FiftyoneDegrees::Common;
using namespace FiftyoneDegrees::DeviceDetection::Hash;

string fileName = "/Users/pholley/Downloads/Enterprise-HashV41.hash";
string propertiesString = "DeviceType,IsSmartPhone,IsTablet,HardwareName,HardwareModel,HardwareVendor,PlatformName,PlatformVersion,BrowserName,BrowserVersion";

Engine::Engine(
        rust::String dataFile,
        DeviceDetection::Hash::ConfigHash *config,
        Common::RequiredPropertiesConfig *properties)
        : hash(EngineHash(fileName, config, properties)) {
}

std::unique_ptr <Result> Engine::process(rust::Str userAgent) const {
    return std::unique_ptr<Result>(new Result(hash.process(userAgent.data())));
}

RequiredPropertiesConfig *properties = new RequiredPropertiesConfig(&propertiesString);

std::unique_ptr <Engine> new_engine_hash(rust::String arg) {
    ConfigHash *config = new ConfigHash();
    config->setMaxPerformance();

    return std::unique_ptr<Engine>(new Engine(fileName, config, properties));
}

Result::Result(ResultsHash* hash): hash(hash) {
}

rust::String Result::getValueAsString(rust::Str propertyName) const {
    Value <string> value = hash->getValueAsString(std::string(propertyName));

    if (value.hasValue()) {
        return value.getValue();
    }

    // https://github.com/dtolnay/cxx/issues/87
    throw std::invalid_argument("Could not find a value for the provided propertyName.");
}

bool Result::getValueAsBool(rust::Str propertyName) const {
    Value <bool> value = hash->getValueAsBool(std::string(propertyName));

    if (value.hasValue()) {
        return value.getValue();
    }

    // https://github.com/dtolnay/cxx/issues/87
    throw std::invalid_argument("Could not find a value for the provided propertyName.");
}

int Result::getValueAsInteger(rust::Str propertyName) const {
    Value <int> value = hash->getValueAsInteger(std::string(propertyName));

    if (value.hasValue()) {
        return value.getValue();
    }

    // https://github.com/dtolnay/cxx/issues/87
    throw std::invalid_argument("Could not find a value for the provided propertyName.");
}

double Result::getValueAsDouble(rust::Str propertyName) const {
    Value <double> value = hash->getValueAsDouble(std::string(propertyName));

    if (value.hasValue()) {
        return value.getValue();
    }

    // https://github.com/dtolnay/cxx/issues/87
    throw std::invalid_argument("Could not find a value for the provided propertyName.");
}