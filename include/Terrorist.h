//
// Created by moosavi on 10/19/22.
//

#ifndef COUNTER_STRIKE_QUERA_TERRORIST_H
#define COUNTER_STRIKE_QUERA_TERRORIST_H

#include "Team.h"

class Terrorist : public Team {
    Terrorist() : Team(GlobalVariable::access_level::terrorist) {}
};

#include "../src/Terrorist.h"

#endif //COUNTER_STRIKE_QUERA_TERRORIST_H
