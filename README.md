# Klawiszologia

edytowanie wielokąta: 
rysowanie - klikamy po kolei na ekran

przesuwanie punktu - klikamy na punkt by go zaznaczyc, punkt podąża za myszką
przesuwanie krawędzi - klikamy na krawędź by ją zaznaczyć, krawędź podąża za myszką
przesuwanie wielokąta - klikamy wewnątrz wielokąta, wielokąt podąża za myszką

dodawanie punktu do odcinka - zaznaczamy odcinek, wciskamy A
usuwanie punktu - zaznaczamy punkt, wciskamy D

dodanie restrykcji pionowej - zaznaczamy odcinek, wcisamy V (wcisniecie na odcinku z zaloza restrykcja powoduje zdjęcie restrykcji lub jej zmianę)
dodanie restrykcji poziomej - zaznaczmy odcinek, wciskamy H (wcisniecie na odcinku z zaloza restrykcja powoduje zdjęcie restrykcji lub jej zmianę)

wybór algorytmu - radio button

rysowanie obwówdki - przycisk B
zmienianie offsetu - strzałki w dół i w górę.


# przyjęte założenia

# algorytm relacji

Relacja - struktura trzymająca krawędź oraz kierunek 

podczas dodawania relacji, sprawdzamy czy dodanie jej jest możlwe, przechodząc po wszystkich relacjach i sprawdzając czy krawędzie tego wierzchołka już nie należa do żądanej relacji

gdy relacja zostaje dodana, poprawiana jest pozycja wierzchołka kończącego krawędź (może to pociągnąć kolejne przesunięcia ze sobą)

podczas każdego poruszenia krawędzią lub punktem, poprawiane są pozycje wierzchołków związanych relacją z przesuwanym wierzchołkiem lub krawędzią.

# algorytm wyznaczania wielokąta odsuniętego

dla każdego punktu wielokąta, chcemy znaleźć punkt który będzie odpowiadał przecięciu się linii równoległych do odcinków := (e1, e2) na które składa się dany punkt
przesuniętych o zadany offset.

Znajdujemy więc dla każdego punktu wektor, który dla danego punktu zwraca mu punkt odpowiadający mu na otoczce:
wiemy, że kierunkiem tego wektora będzie suma wektorów normalnych do e1 oraz e2 zwróconych na zewnątrz wielokąta

pozostaje nam wyznaczenie jego długości oznaczmy ją L:

z trygonometrii widać że L = t/cos(alfa/2) (gdzie alfa to kąt między normalnymi)

przekształcając równanie dochodzimy do wzoru
L = t / sqrt((1 + dot_product(n1,n2) ) /2 )

wtedy mnożąc znormalizowaną sumę wektorów n1 oraz n2 przez L otrzymujemy poszukiwany przez nas wetkor.

po dodaniu do każdego punktu otrzymujemy punkt odpowiadający mu na otoczce.

tak otrzymaną otoczkę poprawiamy następująco:

zaczynając od pierwszego punktu otoczki łączymy je z następnymi dopóki połączenie ich nie spowoduje samoprzecięcia lub przecięcia z podstawowym wielokątem,
gdy występuje przecięcie szukamy pierwszego wierzchołka dla którego ono nie wystąpi, ominięte wierzcholki odkładamy do zbioru wierzcholków pozostałych,
gdy dojdziemy do konca obwódki sprawdzamy czy przypadkiem nasza ostatnia krawędz między (0,n) nie spowoduję przecięcia gdy powoduje odrzucamy ostatni do pozostałych i powtarzamy operację dopóki nie będzie okej. tak otrzymujemy pierwszą składową, operację ponawiamy dla wierzchołków ze zbioru pozostałych dopóki ich ilość nie będzie mniejsza niż 3.



