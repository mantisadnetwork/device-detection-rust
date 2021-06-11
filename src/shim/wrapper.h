#include "../../device-detection-cxx/src/fiftyone.h"
#include "../../device-detection-cxx/src/hash/fiftyone.h"
#include "../../device-detection-cxx/src/common-cxx/fiftyone.h"

typedef struct {
    fiftyoneDegreesStatusCode status;
    fiftyoneDegreesResourceManager manager;
    fiftyoneDegreesDataSetBase* dataset;
} fiftyoneDegreesManagerInit;

fiftyoneDegreesManagerInit fiftyoneDegreesManagerInitFile(const char *properties, const char *fileName);

fiftyoneDegreesDataSetBase* initToDataset(const fiftyoneDegreesManagerInit *init);

int mapPropertyToIndex(const fiftyoneDegreesManagerInit *init, const char *property);

typedef struct {
    fiftyoneDegreesResultsHash* results;
} fiftyoneDegreesManagerResults;

void freeResults(fiftyoneDegreesManagerResults results);
void freeInit(fiftyoneDegreesManagerInit init);

fiftyoneDegreesManagerResults initToResults(const fiftyoneDegreesManagerInit *init, const char* userAgent);

const char* getResultsValue(const fiftyoneDegreesManagerResults *results, const int requiredPropertyIndex);






