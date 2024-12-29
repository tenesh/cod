<?php

namespace App\Enums;

enum Gender: string
{
    case Male = 'male';
    case Female = 'female';
    case Other = 'other';

    /**
     * Return all values in array.
     */
    public static function values(): array
    {
        return array_column(self::cases(), 'value');
    }
}
