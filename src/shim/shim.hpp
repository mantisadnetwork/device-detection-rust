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
    ~Result();

    rust::String getValueAsString(int propertyName) const;

    bool getValueAsBool(int propertyName) const;

    int getValueAsInteger(int propertyName) const;

    double getValueAsDouble(int propertyName) const;
};

class Engine : public EngineHash {
    public:
        Engine(
                rust::Str dataFile,
                DeviceDetection::Hash::ConfigHash *config,
                Common::RequiredPropertiesConfig *required,
                rust::Vec <rust::Str> properties);

        rust::Vec<int> indexes() const;

        std::unique_ptr <Result> lookup(rust::Str userAgent) const;

    protected:
        rust::Vec<rust::Str> properties;
};

std::unique_ptr <Engine> new_engine(rust::Str dataFile, rust::Vec<rust::Str> properties);