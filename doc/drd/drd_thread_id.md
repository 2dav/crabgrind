Retrieves the DRD-specific thread ID for the current thread context.

Executes the `DRD_GET_DRD_THREADID` client request to retrieve the thread ID assigned by 
DRD to the current context.

The returned ID matches the thread IDs reported in DRD data race reports
and trace messages, making it useful for correlating log output with
internal state.

# Mechanics
Unlike the core Valgrind thread IDs, DRD IDs have specific constraints:
* **1-based:** The first valid ID is 1.
* **Stable:** IDs are **never recycled**. When a thread exits, its ID is not reused. 
  This ensures the ID remains consistent throughout the execution log.

## Note
Requires Valgrind **3.3** (2007) or higher.
