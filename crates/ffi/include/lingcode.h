#ifndef LINGCODE_H
#define LINGCODE_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Create a new input method engine
 *
 * # Safety
 * The returned pointer must be freed with `lingcode_engine_free`
 */
void *lingcode_engine_new(void);

/**
 * Create engine with Rime dictionaries
 */
void *lingcode_engine_with_dicts(const char *dict_dir);

/**
 * Free the input method engine
 *
 * # Safety
 * `engine` must be a valid pointer returned by `lingcode_engine_new`
 */
void lingcode_engine_free(void *engine);

/**
 * Process a key event
 *
 * # Arguments
 * * `engine` - The engine handle
 * * `key` - The key character (ASCII lowercase letter)
 *
 * # Returns
 * 1 if the key was consumed, 0 otherwise
 */
int lingcode_process_key(void *engine, char key);

/**
 * Get the current input buffer
 *
 * # Safety
 * The returned string must be freed with `lingcode_string_free`
 */
char *lingcode_get_buffer(void *engine);

/**
 * Get the number of candidates
 */
int lingcode_get_candidate_count(void *engine);

/**
 * Get a candidate by index
 *
 * # Safety
 * The returned string must be freed with `lingcode_string_free`
 */
char *lingcode_get_candidate(void *engine, int index);

/**
 * Select a candidate by index and return the committed text
 *
 * # Safety
 * The returned string must be freed with `lingcode_string_free`
 */
char *lingcode_select_candidate(void *engine, int index);

/**
 * Handle backspace
 *
 * # Returns
 * 1 if backspace was handled, 0 if buffer was empty
 */
int lingcode_backspace(void *engine);

/**
 * Clear the input state
 */
void lingcode_clear(void *engine);

/**
 * Free a string returned by the API
 *
 * # Safety
 * `s` must be a string returned by one of the API functions
 */
void lingcode_string_free(char *s);

#endif  /* LINGCODE_H */
