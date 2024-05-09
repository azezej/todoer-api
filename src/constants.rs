// Messages
pub const MESSAGE_OK: &str = "ok";
pub const MESSAGE_CAN_NOT_FETCH_DATA: &str = "Can not fetch data";
pub const MESSAGE_CAN_NOT_INSERT_DATA: &str = "Can not insert data";
pub const MESSAGE_CAN_NOT_UPDATE_DATA: &str = "Can not update data";
pub const MESSAGE_CAN_NOT_DELETE_DATA: &str = "Can not delete data";
pub const MESSAGE_SIGNUP_SUCCESS: &str = "Signup successfully";
pub const MESSAGE_SIGNUP_FAILED: &str = "Error while signing up, please try again";
pub const MESSAGE_LOGIN_SUCCESS: &str = "Login successfully";
pub const MESSAGE_LOGIN_FAILED: &str = "Wrong username or password, please try again";
pub const MESSAGE_USER_NOT_FOUND: &str = "User not found, please signup";
pub const MESSAGE_LOGOUT_SUCCESS: &str = "Logout successfully";
pub const MESSAGE_PROCESS_TOKEN_ERROR: &str = "Error while processing token";
pub const MESSAGE_INVALID_TOKEN: &str = "Invalid token, please login again";
pub const MESSAGE_INTERNAL_SERVER_ERROR: &str = "Internal Server Error";

// App related
pub const MESSAGE_TASKS_NOT_FOUND_FOR_USER: &str = "This user doesn't have any task.";
pub const MESSAGE_CAN_NOT_CREATE_TASK: &str = "Error while creating task for user.";
pub const MESSAGE_CAN_NOT_GET_TASK_BY_ID: &str = "Error while trying to get task by id for user.";
pub const MESSAGE_CAN_NOT_DELETE_TASK: &str = "Error while trying to delete task for user.";
pub const MESSAGE_CAN_NOT_UPDATE_TASK_NAME: &str = "Error while trying to update task name.";
pub const MESSAGE_CAN_NOT_UPDATE_TASK_STATUS: &str = "Error while trying to update task status.";
pub const MESSAGE_CAN_NOT_UPDATE_TASK_DONE: &str = "Error while trying to update task done status.";
pub const MESSAGE_CAN_NOT_UPDATE_TASK_TODO_LIST: &str = "Error while trying to update task todo list.";
pub const MESSAGE_CAN_NOT_UPDATE_TASK_DUE_DATE: &str = "Error while trying to update task due date.";
pub const MESSAGE_CAN_NOT_UPDATE_TASK_DESCRIPTION: &str = "Error while trying to update task description.";


// Bad request messages
pub const MESSAGE_TOKEN_MISSING: &str = "Token is missing";
pub const MESSAGE_BAD_REQUEST: &str = "Bad Request";

// Headers
pub const AUTHORIZATION: &str = "Authorization";

// Misc
pub const EMPTY: &str = "";

// ignore routes
pub const IGNORE_ROUTES: [&str; 3] = ["/api/ping", "/users/auth/register", "/users/auth/login"];

// Default number of items per page
pub const DEFAULT_PER_PAGE: i64 = 10;

// Default page number
pub const DEFAULT_PAGE_NUM: i64 = 1;

pub const EMPTY_STR: &str = "";

//Session key
pub const SESSION_SERVER_PUBLIC_KEY: &str = "spk";
pub const SESSION_CLIENT_PUBLIC_KEY: &str = "cpk";
