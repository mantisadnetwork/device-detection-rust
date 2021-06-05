#include "device-detection-cxx/src/hash/EngineHash.hpp"
#include "rust/cxx.h"
#include "shim.hpp"
#include <numeric>
#include <iostream>

using namespace FiftyoneDegrees::Common;
using namespace FiftyoneDegrees::DeviceDetection::Hash;

Engine::Engine(
        rust::Str dataFile,
        DeviceDetection::Hash::ConfigHash *config,
        Common::RequiredPropertiesConfig *required,
        rust::Vec <rust::Str> properties
        )
        : EngineHash(std::string(dataFile), config, required), properties(properties) {

}

rust::Vec<int> Engine::indexes() const {
    rust::Vec<int> mapping;

    fiftyoneDegreesDataSetBase* dataSet = fiftyoneDegreesDataSetGet(manager.get());

    for (rust::Str property : properties){
        int index = fiftyoneDegreesPropertiesGetRequiredPropertyIndexFromName(dataSet->available, std::string(property).c_str());

        mapping.push_back(index);
    }

    return mapping;
}

std::unique_ptr <Result> Engine::lookup(rust::Str userAgent) const {
    return std::unique_ptr<Result>(new Result(process(std::string(userAgent).c_str())));
}

std::unique_ptr <Engine> new_engine(rust::Str dataFile, rust::Vec <rust::Str> properties) {
    ConfigHash *config = new ConfigHash();
    config->setMaxPerformance();

    string requiredString = std::accumulate(std::begin(properties), std::end(properties), string(),
                                      [](string &ss, rust::Str &s) {
                                          return ss.empty() ? std::string(s) : ss + "," + std::string(s);
                                      });


    RequiredPropertiesConfig *required = new RequiredPropertiesConfig(&requiredString);

    return std::unique_ptr<Engine>(new Engine(dataFile, config, required, properties));
}

Result::Result(ResultsHash *hash) : hash(hash) {
}

rust::String Result::getValueAsString(int propertyName) const {
    Value <string> value = hash->getValueAsString(propertyName);

    if (value.hasValue()) {
        return value.getValue();
    }

    // https://github.com/dtolnay/cxx/issues/87
    throw std::invalid_argument("Could not find a value for the provided propertyName.");
}

bool Result::getValueAsBool(int propertyName) const {
    Value<bool> value = hash->getValueAsBool(propertyName);

    if (value.hasValue()) {
        return value.getValue();
    }

    // https://github.com/dtolnay/cxx/issues/87
    throw std::invalid_argument("Could not find a value for the provided propertyName.");
}

int Result::getValueAsInteger(int propertyName) const {
    Value<int> value = hash->getValueAsInteger(propertyName);

    if (value.hasValue()) {
        return value.getValue();
    }

    // https://github.com/dtolnay/cxx/issues/87
    throw std::invalid_argument("Could not find a value for the provided propertyName.");
}

double Result::getValueAsDouble(int propertyName) const {
    Value<double> value = hash->getValueAsDouble(propertyName);

    if (value.hasValue()) {
        return value.getValue();
    }

    // https://github.com/dtolnay/cxx/issues/87
    throw std::invalid_argument("Could not find a value for the provided propertyName.");
}