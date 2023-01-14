# Лабораторная работа №13

**Установка docker и docker-compose**

```bash
$ sudo pacman -S docker
$ sudo pacman -S docker-compose
```

**Запуск docker-контейнеров**

```bash
$ cd {path_to_project}
$ docker-compose up
```

**Проверка работоспособности**

В новом терминале стучимся к приложению:
```bash
$ curl http://0.0.0.0/v1/todos/63443a02-2137-48e8-8db5-79fa54e8bfdf
```
Если получили ответ:
```bash
> {"assigned":"user@example.com","id":"63443a02-2137-48e8-8db5-79fa54e8bfdf","message":"Just do it!","priority":"A"}
```
то всё ок