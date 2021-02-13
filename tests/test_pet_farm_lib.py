from python_to_rust import pet_farm


def test_pet_farm():
    cat = pet_farm.Cat('Murka')
    dog = pet_farm.Dog('Drugok')
    assert cat.say() == 'Meow'
    assert dog.say() == 'Woof'

    cat2 = pet_farm.Cat('Miliska')
    assert cat is not cat2

    assert cat.name == 'Murka'
    assert dog.name == 'Drugok'
