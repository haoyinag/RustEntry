<!-- @format -->

## 步骤

1、当前项目目录新增`.vscode`文件夹
2、`.vscode`下新增`launch.json`和`tasks.json`两个文件
3、`launch.json`

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "name": "(Windows) 启动",
      "preLaunchTask": "build",
      "type": "cppvsdbg",
      "request": "launch",
      "program": "${workspaceFolder}/target/debug/${workspaceFolderBasename}.exe",
      "args": [],
      "stopAtEntry": false,
      "cwd": "${workspaceFolder}",
      "environment": [],
      "externalConsole": false
    },
    {
      "name": "(gdb) 启动",
      "type": "cppdbg",
      "request": "launch",
      "program": "${workspaceFolder}/target/debug/${workspaceFolderBasename}.exe",
      "args": [],
      "stopAtEntry": false,
      "cwd": "${workspaceFolder}",
      "environment": [],
      "externalConsole": false,
      "MIMode": "gdb",
      // "miDebuggerPath": "这里填GDB所在的目录",
      "setupCommands": [
        {
          "description": "为 gdb 启用整齐打印",
          "text": "-enable-pretty-printing",
          "ignoreFailures": true
        }
      ]
    }
  ]
}
```

4、`tasks.json`

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "build",
      "type": "shell",
      "command": "cargo",
      "args": ["build"]
    }
  ]
}
```

5、按`F5`启动调试
注意，当前项目必须是根目录
