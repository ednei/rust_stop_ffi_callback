#include <iostream>
#include "jigsaw.h"

using namespace std;


void escape(){
    cout << "Stupid guy on cpp: Well, I just do the stupid thing JigSaw expect me to do!" << endl;
}

int main()
{
    Jigsaw jigsaw(&escape);
    jigsaw.i_want_to_play_a_game();
    cout << "Stupid guy on cpp: I win Jigsaw, who is stupid here?" << endl;
}