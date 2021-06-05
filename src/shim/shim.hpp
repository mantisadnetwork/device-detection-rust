#include "device-detection-cxx/src/hash/EngineHash.hpp"
#include "rust/cxx.h"
#include "device-detection-cxx/src/hash/ResultsHash.hpp"

using namespace FiftyoneDegrees::Common;
using namespace FiftyoneDegrees::DeviceDetection;
using namespace FiftyoneDegrees::DeviceDetection::Hash;

class Result {
    ResultsHash* hash;
public:
    Result(ResultsHash* hash);

    rust::String getValueAsString(rust::Str propertyName) const;

    bool getValueAsBool(rust::Str propertyName) const;

    int getValueAsInteger(rust::Str propertyName) const;

    double getValueAsDouble(rust::Str propertyName) const;
};

class Engine {
    EngineHash hash;
public:
    Engine(
            rust::String dataFile,
            DeviceDetection::Hash::ConfigHash *config,
            Common::RequiredPropertiesConfig *properties);

    std::unique_ptr <Result> process(rust::Str arg) const;
};

std::unique_ptr <Engine> new_engine_hash(rust::String arg);