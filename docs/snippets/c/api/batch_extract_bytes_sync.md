```c title="C"
#include "kreuzberg.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
    const char *html = "<html><body><p>Hello</p></body></html>";
    const char *csv = "name,age\nAlice,30\nBob,25";

    const uint8_t *data[] = {(const uint8_t *)html, (const uint8_t *)csv};
    uintptr_t lengths[] = {strlen(html), strlen(csv)};
    const char *mime_types[] = {"text/html", "text/csv"};
    uintptr_t count = 2;

    struct CBatchResult *batch = kreuzberg_batch_extract_bytes_sync(
        data, lengths, mime_types, count, NULL);
    if (!batch) {
        fprintf(stderr, "Batch error: %s\n", kreuzberg_get_error_details().message);
        return 1;
    }

    for (uintptr_t i = 0; i < batch->count; i++) {
        struct CExtractionResult *r = batch->results[i];
        if (r && r->success) {
            printf("--- Document %zu ---\n%s\n", (size_t)(i + 1), r->content);
        }
    }

    kreuzberg_free_batch_result(batch);
    return 0;
}
```
