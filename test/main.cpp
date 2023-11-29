#include "sclib.h"

#include <iostream>

sclib::Result thingy_function() {
    std::string tank = sclib::prompt_str("WHats your favorite tank");

    //? instead of int you can use u_int16_t and other number types!
    int bitches = sclib::prompt_int<int>("How many bitches do you have");

    if (bitches > 0) {
        std::printf("You have %d bitches? thats impossible!", bitches);
        return sclib::ERR; //? Something went wrong so return ERR
    }

    // Or use the stupidest console io functions ive ever seen
    // std::cout << "You have a " << tank << " and yet you have " << bitches << "bitches!" << std::endl; //? Was it worth the brain cell loss to use this?

    std::printf("You have a %s and yet have %d bitches!\n", tank.c_str(), bitches);

    return sclib::OK; //? Everything went okay
}


int main() {
    sclib::tasker.add_task(1, thingy_function); //? Add a new task with the id 1

    // sclib::tasker.run_task(1); //? run one task, mostly for debugging purposes

    sclib::tasker.run_all_tasks(); //? Run all existing tasks
}

