#ifndef CALLBACK_H
#define CALLBACK_H

#include <stddef.h>

#if defined(_WIN32) || defined(_WIN64)
  #ifdef CALLBACK_EXPORTS
    #define CALLBACK_API __declspec(dllexport)
  #else
    #define CALLBACK_API __declspec(dllimport)
  #endif
#else
  #define CALLBACK_API
#endif

#ifdef __cplusplus
extern "C"
{
#endif

  /**
   * Processes input asynchronously and calls the provided callback with the result
   * @param input The input string to process
   * @param callback Function pointer to call with the processed result
   * @param userdata User-provided data to pass through to the callback
   */
  CALLBACK_API void async_process(
      const char *input,
      size_t (*callback)(const char *, size_t, void *),
      void *userdata);

#ifdef __cplusplus
}
#endif

#endif /* CALLBACK_H */
