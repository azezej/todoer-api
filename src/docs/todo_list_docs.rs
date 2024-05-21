use utoipa::OpenApi;
use crate::{error::ServiceError, models::{dto::todo_list::*, response::*, todo_list::*, db::todo_list::*}, routes::todo_list::*};

#[derive(OpenApi)]
#[openapi(
    paths(create_list, get_lists, get_list_by_id, delete_list, patch_list_title, patch_list_description, patch_list_shared_with, patch_list_parent_list_id),
    components(
        schemas(
            TodoListDTO, 
            TodoList, 
            UpdateTodoListTitleDTO, 
            UpdateTodoListDescriptionDTO, 
            UpdateTodoListSharedWithDTO,
            UpdateTodoListParentListIdDTO,
            UpdateTodoListTitleDB,
            UpdateTodoListDescriptionDB,
            UpdateTodoListSharedWithDB,
            UpdateTodoListParentListIdDB,
            InternalServerErrorResponse,
            TokenMissingResponse,
            BadRequestResponse,
            TodoListResponse,
            AllTodoListsResponse,
            TodoListDeletedResponse, 
            ServiceError,
        ),
        responses(
            InternalServerErrorResponse,
            TokenMissingResponse,
            BadRequestResponse,
            TodoListResponse,
            AllTodoListsResponse,
            TodoListDeletedResponse, 
            ServiceError,
        ),
    ),
)]
pub struct TodoListApi;