#include <stdio.h>
#include <string.h>

void async_process(const char *input, size_t (*callback)(const char *, size_t, void *), void *userdata)
{
  const char *response = "This is a processed response";
  callback(response, strlen(response), userdata);
}
