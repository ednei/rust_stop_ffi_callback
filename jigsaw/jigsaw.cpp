#include <iostream>
#include <cstdlib>
#include <thread>
#include "jigsaw.h"


using namespace std;

Jigsaw::Jigsaw(EscapeCallback callback){
     escapeCallback = callback;
}

Jigsaw::~Jigsaw(){
}

void game_run(EscapeCallback escapeCallback)
{
     cout << "JigSaw: I want to play a game!!!" << endl;
     escapeCallback();
     cout << "JigSaw: you failed!! And now, you DIE!!" << endl;
     exit(EXIT_FAILURE);
}


void Jigsaw::i_want_to_play_a_game(){
     std::thread game(game_run,escapeCallback);  
     game.join();
}
