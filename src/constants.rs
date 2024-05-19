// Messages
pub const MESSAGE_OK: &str = "ok";
pub const MESSAGE_SIGNUP_SUCCESS: &str = "Signup successfully";
pub const MESSAGE_LOGIN_SUCCESS: &str = "Login successfully";
pub const MESSAGE_LOGIN_FAILED: &str = "Wrong username or password, please try again";
pub const MESSAGE_USER_NOT_FOUND: &str = "User not found, please signup";
pub const MESSAGE_LOGOUT_SUCCESS: &str = "Logout successfully";
pub const MESSAGE_PROCESS_TOKEN_ERROR: &str = "Error while processing token";
pub const MESSAGE_INVALID_TOKEN: &str = "Invalid token, please login again";
pub const MESSAGE_INTERNAL_SERVER_ERROR: &str = "Internal Server Error";

// App related
pub const MESSAGE_TASKS_NOT_FOUND_FOR_USER: &str = "This user doesn't have any task.";
pub const MESSAGE_CAN_NOT_GET_TASK_BY_ID: &str = "Error while trying to get task by id for user.";
pub const MESSAGE_CAN_NOT_DELETE_TASK: &str = "Error while trying to delete task for user.";
pub const MESSAGE_CAN_NOT_UPDATE_TASK_NAME: &str = "Error while trying to update task name.";
pub const MESSAGE_CAN_NOT_UPDATE_TASK_DONE: &str = "Error while trying to update task done status.";
pub const MESSAGE_CAN_NOT_UPDATE_TASK_TODO_LIST: &str = "Error while trying to update task todo list.";
pub const MESSAGE_CAN_NOT_UPDATE_TASK_DUE_DATE: &str = "Error while trying to update task due date.";
pub const MESSAGE_CAN_NOT_UPDATE_TASK_DESCRIPTION: &str = "Error while trying to update task description.";
pub const MESSAGE_LISTS_NOT_FOUND_FOR_USER: &str = "This user doesn't have any list.";
pub const MESSAGE_CAN_NOT_GET_LIST_BY_ID: &str = "Error while trying to get list by id for user.";
pub const MESSAGE_CAN_NOT_CREATE_LIST: &str = "Error while creating list for user.";
pub const MESSAGE_CAN_NOT_DELETE_LIST: &str = "Error while trying to delete list for user.";
pub const MESSAGE_CAN_NOT_UPDATE_LIST_NAME: &str = "Error while trying to update list name.";
pub const MESSAGE_CAN_NOT_UPDATE_LIST_DESCRIPTION: &str = "Error while trying to update list description.";
pub const MESSAGE_CAN_NOT_UPDATE_LIST_PARENT_LIST_ID: &str = "Error while trying to update list parent list id.";
pub const MESSAGE_CAN_NOT_UPDATE_LIST_SHARED_WITH: &str = "Error while trying to update list shared with.";

// Bad request messages
pub const MESSAGE_TOKEN_MISSING: &str = "Token is missing";

// Headers
pub const AUTHORIZATION: &str = "Authorization";

// Misc
pub const EMPTY: &str = "";

// ignore routes
pub const IGNORE_ROUTES: [&str; 5] = ["/api/ping", "/users/auth/register", "/users/auth/login", "/swagger-ui/", "/api-docs/"];
