#ifndef NYXVION_SHIELD_H
#define NYXVION_SHIELD_H

#ifdef __cplusplus
extern "C" {
#endif

// Opaque struct representing the Nyxvion Shield Engine
typedef struct Engine Engine;

/**
 * Creates a new Nyxvion Shield Engine instance from a string of rules (e.g., EasyList).
 * 
 * @param rules The blocklist/whitelist rules as a null-terminated C string.
 * @return A pointer to the newly created Engine instance, or NULL on failure.
 *         The caller is responsible for freeing this engine using `nyxvion_shield_destroy`.
 */
Engine* nyxvion_shield_create(const char* rules);

/**
 * Checks if a network request should be blocked according to the engine's rules.
 * 
 * @param engine A pointer to the Engine instance.
 * @param url The URL of the network request.
 * @param source_url The URL of the page originating the request.
 * @param request_type The type of the request (e.g., "script", "image", "document").
 * @param method The HTTP method of the request (e.g., "GET", "POST").
 * @return 1 if the request should be blocked, 0 if it is allowed.
 */
int nyxvion_shield_check_request(
    Engine* engine,
    const char* url,
    const char* source_url,
    const char* request_type,
    const char* method
);

/**
 * Gets cosmetic CSS (hidden class/id selectors) for a specific URL/domain to inject into the page.
 * 
 * @param engine A pointer to the Engine instance.
 * @param url The URL of the page.
 * @return A dynamically allocated null-terminated C string containing the CSS.
 *         The caller MUST free this string using `nyxvion_shield_free_string`.
 */
char* nyxvion_shield_get_cosmetic_css(Engine* engine, const char* url);

/**
 * Serializes the current engine rules to a fast-loading binary format (Flatbuffers/DAT).
 * 
 * @param engine A pointer to the Engine instance.
 * @param out_size A pointer to a size_t where the size of the binary data will be written.
 * @return A pointer to the binary data buffer.
 *         The caller MUST free this buffer using `nyxvion_shield_free_binary`.
 */
unsigned char* nyxvion_shield_serialize(Engine* engine, size_t* out_size);

/**
 * Creates a new Nyxvion Shield Engine instantly from a previously serialized binary format.
 * 
 * @param data A pointer to the binary data buffer.
 * @param size The size of the binary data buffer in bytes.
 * @return A pointer to the newly created Engine instance, or NULL on failure.
 */
Engine* nyxvion_shield_create_from_binary(const unsigned char* data, size_t size);

/**
 * Frees a string previously returned by `nyxvion_shield_get_cosmetic_css` or similar functions.
 * 
 * @param str A pointer to the string to free.
 */
void nyxvion_shield_free_string(char* str);

/**
 * Frees a binary buffer previously returned by `nyxvion_shield_serialize`.
 * 
 * @param data A pointer to the buffer to free.
 * @param size The size of the buffer.
 */
void nyxvion_shield_free_binary(unsigned char* data, size_t size);

/**
 * Destroys a Nyxvion Shield Engine instance, freeing its memory.
 * 
 * @param engine A pointer to the Engine instance to destroy.
 */
void nyxvion_shield_destroy(Engine* engine);

#ifdef __cplusplus
}
#endif

#endif // NYXVION_SHIELD_H
