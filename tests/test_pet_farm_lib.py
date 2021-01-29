from python_to_rust import pet_farm


def test_pet_farm():
    cat = pet_farm.Cat()
    dog = pet_farm.Dog()
    assert cat.say() == 'Meow'
    assert dog.say() == 'Woof'

    cat2 = pet_farm.Cat()
    assert cat is not cat2
