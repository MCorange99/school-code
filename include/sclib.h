/**
 * # SCLib, a library to help with the programming class bullshit.
 * 
 * ## NOTE
 * 
 * !!! This probably requires C++ 23 so make sure to set this in your IDE/compiler
 * 
 * ## Adding a task (task_num is the task number and task_fn is the name of the function (the function pointer))
 * ```c++
 * sclib::tasker.add_task(task_num, task_fn);
 * ```
 * 
 * ## Running 1 task
 * ```c++
 * sclib::tasker.run_task(task_num);
 * ```
 * 
 * ## Running all tasks
 * ```c++
 * sclib::tasker.run_all_tasks();
 * ```
 * 
 * ## Other info
 * All tasker functions return `sclib::Result` which is just a `int`
 * The function will return `sclib::OK` when no errors happen.
 * Likewise the function will return `sclib::ERR` when an error happens.
 * 
 * To handle the errors just compare the function return value to either sclib::OK or sclib::ERR depending on the use.
 * Example:
 * 
 * ```c++
 * if (sclib::tasker.run_task(69) == sclib::OK) {
 *     cout << "Task ran okay!";
 * }
 * ```
 * 
 * OR
 * 
 * ```c++
 * if (sclib::tasker.run_task(69) == sclib::ERR) {
 *     cout << "Task had an error! Should probably do something";
 * }
 * ```
 * 
 * ## Authors
 * 
 * MCorange <gvidasjuknevicius2@gmail.com>
*/

#pragma once

#include <iostream>
#include <string>
#include <ranges>
#include <algorithm>
#include <unordered_map>
#include <functional>


//! Helper functions that should not be used by user
static inline void __str_ltrim(std::string &s) {s.erase(s.begin(), std::find_if(s.begin(), s.end(), [](unsigned char ch) {return !std::isspace(ch);}));}
static inline void __str_rtrim(std::string &s) {s.erase(std::find_if(s.rbegin(), s.rend(), [](unsigned char ch) {return !std::isspace(ch);}).base(), s.end()); }
inline void __str_trim(std::string &s) { __str_rtrim(s); __str_ltrim(s); }

namespace sclib {

    using Result = int;
    using TaskFnPtr = Result (*)();

    const int OK = 0;
    const int ERR = 1;
    
    
    class SCLib {
    private:
        std::unordered_map<u_int32_t, TaskFnPtr> tasks;
        u_int32_t last_task;
    public:
        auto add_task(const u_int32_t task_num, TaskFnPtr fn_ptr) -> Result {
            if (tasks.contains(task_num)) {
                std::printf("A task with the number %d already exists!\n", task_num);
                return ERR;
            }

            tasks.insert({task_num, fn_ptr});

            if (task_num > last_task) last_task = task_num;

            return OK;
        }

        auto run_task(u_int32_t task_num) -> Result {
            if (!tasks.contains(task_num)) {
                std::printf("A task with the number %d does not exist!\n", task_num);
                return ERR;
            }
            
            std::printf("--- Task %d start ---\n", task_num);
            const Result ret = (*tasks[task_num])();
            std::printf("--- Task %d end---\n", task_num);
            
            if (ret == ERR) {
                std::printf("ERROR: Task %d failed to execute correctly!\n", task_num);
            }
            return ret;
        }

        auto run_all_tasks() -> Result {

            Result ret = 0;
            u_int32_t i = 0;

            if (!tasks.contains(i)) i = 1; //? In case the tasks start from 1

            

            while (ret == OK) {
                if (i > last_task) {
                    break;
                }
                ret = run_task(i);
                i++;
            }

            std::printf("INFO: Ran all tasks, exiting!\n");
            return ret;
        }

    };
    
    //? Adding at the bottom in case of a shitty compiler
    sclib::SCLib tasker;

    auto read_str() -> std::string {
        std::string buf;
        getline(std::cin, buf);
        return buf;
    }

    template <typename T>
    auto read_int() -> T {
        std::string inp = read_str(); 
        __str_trim(inp);
        return atoi(inp.c_str());
    }


    auto prompt_str(const std::string prompt) -> std::string {
        std::printf("%s: ", prompt.c_str());
        return read_str();
    }

    template <typename T>
    auto prompt_int(const std::string prompt) -> T {
        std::printf("%s: ", prompt.c_str());
        return read_int<T>();
    }
};

