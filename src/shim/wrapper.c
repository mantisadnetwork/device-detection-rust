#include "wrapper.h"

fiftyoneDegreesManagerInit fiftyoneDegreesManagerInitFile(const char *properties, const char *fileName) {
    ResourceManager manager;
    fiftyoneDegreesConfigHash config = fiftyoneDegreesHashHighPerformanceConfig;

    fiftyoneDegreesPropertiesRequired required = fiftyoneDegreesPropertiesDefault;
    required.string = properties;

    EXCEPTION_CREATE;
    StatusCode status = HashInitManagerFromFile(
            &manager,
            &config,
            &required,
            fileName,
            exception);
    EXCEPTION_THROW;

    fiftyoneDegreesManagerInit init;
    init.manager = manager;
    init.status = status;
    init.dataset = fiftyoneDegreesDataSetGet(&manager);

    return init;
}

int mapPropertyToIndex(const fiftyoneDegreesManagerInit *init, const char *property) {
    return fiftyoneDegreesPropertiesGetRequiredPropertyIndexFromName(init->dataset->available, property);
}

void freeInit(fiftyoneDegreesManagerInit init) {
    fiftyoneDegreesDataSetFree(init.dataset);
    fiftyoneDegreesResourceManagerFree(&init.manager);
}

fiftyoneDegreesDataSetBase *initToDataset(const fiftyoneDegreesManagerInit *init) {
    // fiftyoneDegreesDataSetGet requires *results but we don't want rust to know about interior mutability
#pragma GCC diagnostic ignored "-Wincompatible-pointer-types-discards-qualifiers"
    return fiftyoneDegreesDataSetGet(&init->manager);
}

fiftyoneDegreesManagerResults initToResults(const fiftyoneDegreesManagerInit *init, const char *userAgent) {
    fiftyoneDegreesManagerResults results;

    // fiftyoneDegreesResultsHashCreate requires *results but we don't want rust to know about interior mutability
#pragma GCC diagnostic ignored "-Wincompatible-pointer-types-discards-qualifiers"
    results.results = fiftyoneDegreesResultsHashCreate(&init->manager, 1, 0);

    EXCEPTION_CREATE;
    fiftyoneDegreesResultsHashFromUserAgent(results.results, userAgent, strlen(userAgent), exception);
    EXCEPTION_THROW;

    return results;
}

void freeResults(fiftyoneDegreesManagerResults results) {
    fiftyoneDegreesResultsHashFree(results.results);
}

const char *getResultsValue(const fiftyoneDegreesManagerResults *results, const int requiredPropertyIndex) {
    EXCEPTION_CREATE;

    if (ResultsHashGetValues(results->results, requiredPropertyIndex, exception) != NULL && EXCEPTION_OKAY) {
        if (results->results->values.count > 0) {
            return FIFTYONE_DEGREES_STRING(results->results->values.items[0].data.ptr);
        }
    }

    EXCEPTION_THROW;

    return NULL;
}