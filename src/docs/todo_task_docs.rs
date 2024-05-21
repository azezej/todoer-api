use utoipa::OpenApi;
use crate::{error::ServiceError, models::{dto::todo_task::*, response::*, todo_task::*, db::todo_task::*}, routes::todo_task::*};

#[derive(OpenApi)]
#[openapi(
    paths(create_task, get_tasks, get_task_by_id, delete_task, patch_task_summary, patch_task_description, patch_task_due_date, patch_task_todolist_id, patch_task_parent_task_id),
    components(
        schemas(
            TodoTaskDTO, 
            TodoTask, 
            UpdateTodoTaskSummaryDTO, 
            UpdateTodoTaskDescriptionDTO, 
            UpdateTodoTaskParentTaskDTO, 
            UpdateTodoTaskDueDateDTO, 
            UpdateTodoTaskTodoListDTO, 
            UpdateTodoTaskDoneDTO,
            UpdateTodoTaskSummaryDB, 
            UpdateTodoTaskDescriptionDB, 
            UpdateTodoTaskParentTaskDB, 
            UpdateTodoTaskDueDateDB, 
            UpdateTodoTaskTodoListDB, 
            UpdateTodoTaskDoneDB,
            InternalServerErrorResponse,
            TokenMissingResponse,
            BadRequestResponse,
            TodoTaskResponse,
            AllTodoTasksResponse,
            TodoTaskDeletedResponse, 
            ServiceError,
        ),
        responses(
            InternalServerErrorResponse,
            TokenMissingResponse,
            BadRequestResponse,
            TodoTaskResponse,
            AllTodoTasksResponse,
            TodoTaskDeletedResponse, 
            ServiceError,
        ),
    ),
)]
pub struct TodoTaskApi;