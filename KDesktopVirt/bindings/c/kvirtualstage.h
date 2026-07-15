/**
 * KVirtualStage C/C++ API Header
 * 
 * Playwright-equivalent desktop automation platform for AI agents.
 * Provides C FFI bindings for native integration in C/C++ applications.
 * 
 * Usage Example:
 * ```c
 * #include "kvirtualstage.h"
 * 
 * int main() {
 *     // Initialize KVirtualStage
 *     if (kvs_init() != 0) {
 *         fprintf(stderr, "Failed to initialize KVirtualStage\n");
 *         return 1;
 *     }
 *     
 *     // Create session
 *     char session_id[256];
 *     if (kvs_create_session("demo_user", "my_session", "ubuntu", 
 *                           session_id, sizeof(session_id)) != 0) {
 *         fprintf(stderr, "Failed to create session\n");
 *         return 1;
 *     }
 *     
 *     printf("Session created: %s\n", session_id);
 *     
 *     // Perform automation
 *     kvs_move_cursor(session_id, 400.0, 300.0);
 *     kvs_click(session_id, "left");
 *     kvs_type_text(session_id, "Hello from KVirtualStage!");
 *     
 *     // Start recording
 *     char recording_id[256];
 *     kvs_start_recording(session_id, "demo.mp4", "medium", 
 *                        recording_id, sizeof(recording_id));
 *     
 *     // Stop recording
 *     char output_path[512];
 *     kvs_stop_recording(session_id, output_path, sizeof(output_path));
 *     printf("Recording saved: %s\n", output_path);
 *     
 *     // Cleanup
 *     kvs_shutdown();
 *     return 0;
 * }
 * ```
 */

#ifndef KVIRTUALSTAGE_H
#define KVIRTUALSTAGE_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <stdbool.h>

/* ============================================================================
 * Constants and Error Codes
 * ============================================================================ */

/** Success return code */
#define KVS_SUCCESS 0

/** Error codes */
#define KVS_ERROR_INVALID_PARAM -1
#define KVS_ERROR_NOT_INITIALIZED -2
#define KVS_ERROR_BUFFER_TOO_SMALL -3
#define KVS_ERROR_OPERATION_FAILED -4
#define KVS_ERROR_SESSION_NOT_FOUND -5
#define KVS_ERROR_RECORDING_ACTIVE -6
#define KVS_ERROR_RECORDING_NOT_ACTIVE -7

/** Maximum string lengths */
#define KVS_MAX_SESSION_ID_LEN 64
#define KVS_MAX_USER_ID_LEN 64
#define KVS_MAX_DESKTOP_TYPE_LEN 32
#define KVS_MAX_STATUS_LEN 32
#define KVS_MAX_FILENAME_LEN 256
#define KVS_MAX_TEXT_LEN 1024
#define KVS_MAX_ERROR_MSG_LEN 512

/* ============================================================================
 * Data Structures
 * ============================================================================ */

/**
 * 2D coordinate point
 */
typedef struct {
    double x;
    double y;
} kvs_point_t;

/**
 * Session information structure
 */
typedef struct {
    char session_id[KVS_MAX_SESSION_ID_LEN];
    char user_id[KVS_MAX_USER_ID_LEN];
    char desktop_type[KVS_MAX_DESKTOP_TYPE_LEN];
    char status[KVS_MAX_STATUS_LEN];
    uint64_t created_at;
    uint64_t last_activity;
    bool recording_active;
} kvs_session_info_t;

/**
 * Workflow execution result
 */
typedef struct {
    char workflow_name[KVS_MAX_FILENAME_LEN];
    bool success;
    uint32_t total_steps;
    uint32_t successful_steps;
    uint64_t execution_time_ms;
    uint32_t error_count;
    char errors[10][KVS_MAX_ERROR_MSG_LEN]; // Max 10 errors
} kvs_workflow_result_t;

/**
 * Mouse button types
 */
typedef enum {
    KVS_MOUSE_LEFT,
    KVS_MOUSE_RIGHT,
    KVS_MOUSE_MIDDLE
} kvs_mouse_button_t;

/**
 * Desktop types
 */
typedef enum {
    KVS_DESKTOP_UBUNTU,
    KVS_DESKTOP_UBUNTU_XFCE,
    KVS_DESKTOP_UBUNTU_KDE,
    KVS_DESKTOP_CENTOS,
    KVS_DESKTOP_FEDORA,
    KVS_DESKTOP_ARCH,
    KVS_DESKTOP_DEBIAN
} kvs_desktop_type_t;

/**
 * Recording quality levels
 */
typedef enum {
    KVS_QUALITY_LOW,
    KVS_QUALITY_MEDIUM,
    KVS_QUALITY_HIGH,
    KVS_QUALITY_STREAMING
} kvs_recording_quality_t;

/* ============================================================================
 * Core API Functions
 * ============================================================================ */

/**
 * Initialize KVirtualStage API
 * Must be called before any other functions.
 * 
 * @return KVS_SUCCESS on success, error code on failure
 */
int kvs_init(void);

/**
 * Shutdown and cleanup KVirtualStage API
 * Should be called when done using the API.
 * 
 * @return KVS_SUCCESS on success, error code on failure
 */
int kvs_shutdown(void);

/**
 * Get the last error message
 * 
 * @param buffer Buffer to store error message
 * @param buffer_size Size of the buffer
 * @return KVS_SUCCESS on success, error code on failure
 */
int kvs_get_last_error(char* buffer, uint32_t buffer_size);

/* ============================================================================
 * Session Management Functions
 * ============================================================================ */

/**
 * Create a new automation session
 * 
 * @param user_id User identifier
 * @param session_name Name for the session
 * @param desktop_type Desktop environment type
 * @param result_buffer Buffer to store session ID
 * @param buffer_size Size of result buffer
 * @return KVS_SUCCESS on success, error code on failure
 */
int kvs_create_session(const char* user_id,
                      const char* session_name,
                      const char* desktop_type,
                      char* result_buffer,
                      uint32_t buffer_size);

/**
 * Get information about a session
 * 
 * @param session_id Session identifier
 * @param session_info Pointer to session info structure
 * @return KVS_SUCCESS on success, error code on failure
 */
int kvs_get_session_info(const char* session_id,
                        kvs_session_info_t* session_info);

/**
 * List all active sessions
 * 
 * @param sessions Array to store session info
 * @param max_sessions Maximum number of sessions to return
 * @param actual_count Pointer to store actual number of sessions returned
 * @return KVS_SUCCESS on success, error code on failure
 */
int kvs_list_sessions(kvs_session_info_t* sessions,
                     uint32_t max_sessions,
                     uint32_t* actual_count);

/**
 * Remove/close a session
 * 
 * @param session_id Session identifier
 * @return KVS_SUCCESS on success, error code on failure
 */
int kvs_remove_session(const char* session_id);

/* ============================================================================
 * Automation Control Functions
 * ============================================================================ */

/**
 * Move cursor to specified coordinates with natural movement
 * 
 * @param session_id Session identifier
 * @param target_x Target X coordinate
 * @param target_y Target Y coordinate
 * @return KVS_SUCCESS on success, error code on failure
 */
int kvs_move_cursor(const char* session_id,
                   double target_x,
                   double target_y);

/**
 * Click at current cursor position
 * 
 * @param session_id Session identifier
 * @param button Mouse button to click ("left", "right", "middle", or NULL for left)
 * @return KVS_SUCCESS on success, error code on failure
 */
int kvs_click(const char* session_id,
             const char* button);

/**
 * Click at specific coordinates
 * 
 * @param session_id Session identifier
 * @param x X coordinate
 * @param y Y coordinate
 * @param button Mouse button to click
 * @return KVS_SUCCESS on success, error code on failure
 */
int kvs_click_at(const char* session_id,
                double x,
                double y,
                kvs_mouse_button_t button);

/**
 * Double-click at current cursor position
 * 
 * @param session_id Session identifier
 * @return KVS_SUCCESS on success, error code on failure
 */
int kvs_double_click(const char* session_id);

/**
 * Type text with natural human-like timing
 * 
 * @param session_id Session identifier
 * @param text Text to type
 * @return KVS_SUCCESS on success, error code on failure
 */
int kvs_type_text(const char* session_id,
                 const char* text);

/**
 * Type text with specified words per minute
 * 
 * @param session_id Session identifier
 * @param text Text to type
 * @param wpm Words per minute (typing speed)
 * @return KVS_SUCCESS on success, error code on failure
 */
int kvs_type_text_wpm(const char* session_id,
                     const char* text,
                     double wpm);

/**
 * Press a specific key
 * 
 * @param session_id Session identifier
 * @param key Key name (e.g., "Enter", "Tab", "Escape")
 * @return KVS_SUCCESS on success, error code on failure
 */
int kvs_key_press(const char* session_id,
                 const char* key);

/**
 * Press a combination of keys
 * 
 * @param session_id Session identifier
 * @param keys Array of key names
 * @param key_count Number of keys in combination
 * @return KVS_SUCCESS on success, error code on failure
 */
int kvs_key_combination(const char* session_id,
                       const char* const* keys,
                       uint32_t key_count);

/**
 * Scroll in specified direction
 * 
 * @param session_id Session identifier
 * @param direction Scroll direction ("up", "down", "left", "right")
 * @param amount Scroll amount (number of steps)
 * @return KVS_SUCCESS on success, error code on failure
 */
int kvs_scroll(const char* session_id,
              const char* direction,
              int32_t amount);

/* ============================================================================
 * Recording Functions
 * ============================================================================ */

/**
 * Start recording a session
 * 
 * @param session_id Session identifier
 * @param output_filename Output video filename
 * @param quality Recording quality ("low", "medium", "high", "streaming", or NULL for medium)
 * @param result_buffer Buffer to store recording ID
 * @param buffer_size Size of result buffer
 * @return KVS_SUCCESS on success, error code on failure
 */
int kvs_start_recording(const char* session_id,
                       const char* output_filename,
                       const char* quality,
                       char* result_buffer,
                       uint32_t buffer_size);

/**
 * Stop recording a session
 * 
 * @param session_id Session identifier
 * @param result_buffer Buffer to store output file path
 * @param buffer_size Size of result buffer
 * @return KVS_SUCCESS on success, error code on failure
 */
int kvs_stop_recording(const char* session_id,
                      char* result_buffer,
                      uint32_t buffer_size);

/**
 * Check if session is currently recording
 * 
 * @param session_id Session identifier
 * @param is_recording Pointer to store recording status
 * @return KVS_SUCCESS on success, error code on failure
 */
int kvs_is_recording(const char* session_id,
                    bool* is_recording);

/* ============================================================================
 * Screenshot Functions
 * ============================================================================ */

/**
 * Take a screenshot of the session
 * 
 * @param session_id Session identifier
 * @param filename Output filename (or NULL for auto-generated)
 * @param result_buffer Buffer to store actual filename
 * @param buffer_size Size of result buffer
 * @return KVS_SUCCESS on success, error code on failure
 */
int kvs_screenshot(const char* session_id,
                  const char* filename,
                  char* result_buffer,
                  uint32_t buffer_size);

/* ============================================================================
 * Workflow Functions
 * ============================================================================ */

/**
 * Execute a simple workflow from JSON string
 * 
 * @param session_id Session identifier
 * @param workflow_json JSON string defining the workflow
 * @param result Pointer to store execution result
 * @return KVS_SUCCESS on success, error code on failure
 */
int kvs_execute_workflow_json(const char* session_id,
                             const char* workflow_json,
                             kvs_workflow_result_t* result);

/* ============================================================================
 * Utility Functions
 * ============================================================================ */

/**
 * Get API version information
 * 
 * @param buffer Buffer to store version string
 * @param buffer_size Size of buffer
 * @return KVS_SUCCESS on success, error code on failure
 */
int kvs_get_version(char* buffer, uint32_t buffer_size);

/**
 * Health check - verify API is working
 * 
 * @return KVS_SUCCESS if healthy, error code otherwise
 */
int kvs_health_check(void);

/**
 * Convert desktop type enum to string
 * 
 * @param desktop_type Desktop type enum
 * @return String representation or NULL if invalid
 */
const char* kvs_desktop_type_to_string(kvs_desktop_type_t desktop_type);

/**
 * Convert mouse button enum to string
 * 
 * @param button Mouse button enum
 * @return String representation or NULL if invalid
 */
const char* kvs_mouse_button_to_string(kvs_mouse_button_t button);

/**
 * Convert recording quality enum to string
 * 
 * @param quality Recording quality enum
 * @return String representation or NULL if invalid
 */
const char* kvs_recording_quality_to_string(kvs_recording_quality_t quality);

/* ============================================================================
 * Callback Support (Optional)
 * ============================================================================ */

/**
 * Callback function type for session events
 */
typedef void (*kvs_session_callback_t)(const char* session_id, 
                                       const char* event_type,
                                       const char* event_data,
                                       void* user_data);

/**
 * Register callback for session events
 * 
 * @param session_id Session identifier (or NULL for all sessions)
 * @param callback Callback function
 * @param user_data User data passed to callback
 * @return KVS_SUCCESS on success, error code on failure
 */
int kvs_register_callback(const char* session_id,
                         kvs_session_callback_t callback,
                         void* user_data);

/**
 * Unregister callback for session events
 * 
 * @param session_id Session identifier (or NULL for all sessions)
 * @param callback Callback function to remove
 * @return KVS_SUCCESS on success, error code on failure
 */
int kvs_unregister_callback(const char* session_id,
                           kvs_session_callback_t callback);

/* ============================================================================
 * Advanced Features (Optional)
 * ============================================================================ */

/**
 * Set session timeout (automatic cleanup)
 * 
 * @param session_id Session identifier
 * @param timeout_seconds Timeout in seconds (0 to disable)
 * @return KVS_SUCCESS on success, error code on failure
 */
int kvs_set_session_timeout(const char* session_id,
                           uint32_t timeout_seconds);

/**
 * Set natural movement parameters
 * 
 * @param session_id Session identifier
 * @param enabled Enable/disable natural movement
 * @param speed_factor Movement speed factor (1.0 = normal)
 * @return KVS_SUCCESS on success, error code on failure
 */
int kvs_set_natural_movement(const char* session_id,
                            bool enabled,
                            double speed_factor);

/**
 * Set typing parameters
 * 
 * @param session_id Session identifier
 * @param wpm Words per minute
 * @param error_rate Error simulation rate (0.0 - 1.0)
 * @return KVS_SUCCESS on success, error code on failure
 */
int kvs_set_typing_parameters(const char* session_id,
                             double wpm,
                             double error_rate);

#ifdef __cplusplus
}
#endif

#endif /* KVIRTUALSTAGE_H */