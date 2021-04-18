import React, { useState, useEffect, FormEvent } from 'react';
import { render } from 'react-dom';

interface Todo {
    id: number;
    description: string;
    is_complete: boolean;
    issue_date: number;
}

const displayTodoString = (date: Date, description: string): string => {
    return date.getDate() + '/' + (date.getMonth() + 1) + ' (' + date.getHours() + ':' + date.getMinutes() + '): ' + description;
}

const TodoApp = () => {
    const [todos, setTodos] = useState([]);
    const [newDesc, setNewDesc] = useState('');

    useEffect(() => {
        const getTodos = async () => {
            const response = await fetch('http://localhost:8080/todos', {
                method: 'GET',
                mode: 'cors',
            });

            setTodos(await response.json());
        }
        getTodos();
    }, []);

    const postTodo = async () => {
        const response = await fetch('http://localhost:8080/todo', {
            method: 'POST',
            mode: 'cors',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                description: newDesc,
                is_complete: false,
                issue_date: Date.now()
            })
        });

        const data = await response.json();

        setTodos((oldTodos: Todo[]) => [...oldTodos, data]);
    }

    const handleSubmit = (e: FormEvent) => {
        e.preventDefault();

        postTodo();
    }

    const handleToggleComplete = ({ id, description, is_complete }: Todo) => {
        const updateTodo = async () => {
            const res = await fetch(`http://localhost:8080/todo/${id}`, {
                method: 'PUT',
                mode: 'cors',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    description: description,
                    is_complete: !is_complete,
                    issue_date: Date.now()
                })
            })

            const data = await res.json();

            const newTodoArr = todos.map((item) => {
                if (item.id === data.id) {
                    return data;
                }
                return item;
            })

            setTodos(newTodoArr);
        }
        updateTodo();
    }

    const handleDelete = ({ id }: Todo) => {
        const deleteTodo = async () => {
            const res = await fetch(`http://localhost:8080/todo/${id}`, {
                method: 'DELETE',
                mode: 'cors'
            });

            if (res.ok) {
                const newTodosArr = todos.filter((item) => !(item.id === id));

                setTodos(newTodosArr);
            }
        }

        deleteTodo();
    }

    const todosCheckbox = todos.map(
        ({ id, description, is_complete, issue_date }: Todo) => {
            return (
                <li key={id.toString()}>
                    <button type="button" onClick={() => handleToggleComplete({ id, description, is_complete, issue_date })}>
                        {is_complete ? 'Undo' : 'Done'}
                    </button>

                    <span
                        style={{
                            textDecoration: is_complete ? 'line-through' : 'none'
                        }}

                        onClick={() => handleDelete({ id, description, is_complete, issue_date })}
                    >
                        {displayTodoString(new Date(issue_date), description)}
                    </span>
                </li >
            )
        }
    );

    return (
        <div>
            <form>
                <label>
                    New Todo:
                    <input type="text" value={newDesc} onChange={(event) => setNewDesc(event.target.value)} size={80} />
                </label>
                <button onClick={handleSubmit}>Submit</button>
            </form>
            <ol>
                {todosCheckbox}
            </ol>
        </div>
    );
}

render(<TodoApp />, document.getElementById('root'));
