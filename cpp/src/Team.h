#ifndef COUNTER_STRIKE_QUERA_TEAM_H
#define COUNTER_STRIKE_QUERA_TEAM_H

#include <string>
#include <map>
#include <vector>
#include "Gun.h"
#include "Player.h"
#include "Guns.h"
#include "Time.h"

using std::map;
using std::string;
using std::vector;

class Team {
public:
    explicit Team(Gun::access_level accessLevel);

    void add_player(const string &name, const Time &time);

    bool has_player(const string &name) const;

    Player *get_player(const string &name) const;

    void new_round();

    vector<Player *> get_score_board() const;

    bool has_live() const;

    void won() const;

    void lose() const;

    int get_live_num() const;

    ~Team();

private:
    map<string, Player *> players;
    const Gun::access_level ACCESS_LEVEL;
};

#endif
