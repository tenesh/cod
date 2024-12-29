<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Casts\Attribute;
use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Support\Str;

class Account extends Model
{
    use HasFactory, HasUuids;

    /**
     * The storage format of the model's date columns.
     *
     * @var string
     */
    protected $dateFormat = 'U';

    /**
     * The attributes that are mass assignable.
     *
     * @var list<string>
     */
    protected $fillable = [
        'username',
        'first_name',
        'last_name',
        'gender',
        'birth_date',
        'active',
    ];

    /**
     * Get the attributes that should be cast.
     *
     * @return array<string, string>
     */
    protected function casts(): array
    {

        return [
            'birth_date' => 'date',
            'active' => 'boolean',
        ];
    }

    /**
     * Access or mutate the username attribute.
     *
     * @return Attribute
     */
    protected function username(): Attribute
    {

        return Attribute::make(
            get: fn (string $value) => Str::lower($value),
            set: fn (string $value) => Str::lower($value),
        );
    }

    /**
     * Access or mutate the first_name attribute.
     *
     * @return Attribute
     */
    protected function firstName(): Attribute
    {

        return Attribute::make(
            get: fn (?string $value) => Str::ucfirst($value),
            set: fn (?string $value) => Str::lower($value),
        );
    }

    /**
     * Access or mutate the last_name attribute.
     *
     * @return Attribute
     */
    protected function lastName(): Attribute
    {

        return Attribute::make(
            get: fn (?string $value) => Str::ucfirst($value),
            set: fn (?string $value) => Str::lower($value),
        );
    }

    public function user(): BelongsTo
    {
        return $this->belongsTo(User::class);
    }
}
