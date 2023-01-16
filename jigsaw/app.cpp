#include <iostream>
#include "jigsaw.h"

using namespace std;


void escape(){
    cout << "Stupid guy on cpp: Well, I just do the stupid thing JigSaw expect me to do!" << endl;
}

int main()
{
    Jigsaw jigsaw(&escape);
    try
    {
        jigsaw.i_want_to_play_a_game();
    }
    catch (char const* msg)
    {
        std::cout << "msg: " << msg << '\n';
    }
    //jigsaw.i_want_to_play_a_game();
    //cout << "And I survive" << endl;
}