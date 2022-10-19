//
// Created by moosavi on 10/19/22.
//

#ifndef COUNTER_STRIKE_QUERA_TEAM_H
#define COUNTER_STRIKE_QUERA_TEAM_H

#include <string>
#include <map>
#include "Gun.h"
#include "Player.h"
#include <Guns.h>

using std::map;
using std::string;

class Team {
public:
    Team(GlobalVariable::access_level accessLevel) : ACCESS_LEVEL(accessLevel) {}

    void add_player(const string &name);

protected:
    map<string, Player *> players;
    const GlobalVariable::access_level ACCESS_LEVEL;
    int life = 0;
};

#endif //COUNTER_STRIKE_QUERA_TEAM_H
